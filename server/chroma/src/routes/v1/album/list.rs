use crate::routes::appdata::{AlbumIdCache, WebData};
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use crate::routes::v1::PhotoQuality;
use actix_multiresponse::Payload;
use actix_web::web;
use dal::database::{Album, Photo};
use dal::s3::aws_errors::GetObjectErrorKind;
use dal::s3::{S3Error, SdkError};
use dal::storage_engine::{EngineType, StorageEngineError};
use dal::DalError;
use futures::future::{join_all, try_join_all};
use proto::{AlbumWithCoverPhoto, ListAlbumsResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Query {
    #[serde(default)]
    include_cover_photo: bool,
    #[serde(default)]
    quality_preference: PhotoQuality,
}

/// List all known albums
///
/// # Errors
///
/// - If something went wrong
pub async fn list(
    auth: Authorization,
    album_id_cache: web::Data<AlbumIdCache>,
    data: WebData,
    query: web::Query<Query>,
) -> WebResult<Payload<ListAlbumsResponse>> {
    // Fetch only IDs, so we can grab the rest from cache
    let ids = Album::list_ids(&data.db).await?;

    // Fetch cached albums
    let cached_albums = join_all(ids.into_iter().map(|f| {
        let cache = &**album_id_cache;
        async move {
            let album = cache.get(&f).await;
            (f, album)
        }
    }))
    .await;

    // Fetch albums from database of which there is nothing cached
    let fetched_albums = try_join_all(
        cached_albums
            .iter()
            .filter(|(_, cached)| cached.is_none())
            .map(|(id, _)| Album::get_by_id(&data.db, id)),
    )
    .await?
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    // Insert the newly fetched into the cache
    join_all(fetched_albums.iter().map(|album| {
        let cache = &**album_id_cache;
        async move { cache.insert(album.id.clone(), album.clone()).await }
    }))
    .await;

    // Merge the two sets
    let mut albums = [fetched_albums,
        cached_albums
            .into_iter()
            .filter_map(|(_, v)| v)
            .collect::<Vec<_>>()]
    .concat();

    // Check if we should include draft albums
    let include_draft = auth.is_admin
        || auth
            .has_scope(&data.db, "nl.svsticky.chroma.album.list.draft")
            .await?;

    if !include_draft {
        albums.retain(|f| !f.is_draft);
    }

    // Transform them all to the proto response
    let albums = join_all(albums.into_iter().map(|album| {
        let storage = data.storage.clone();
        let database = data.db.clone();
        let qpref: dal::storage_engine::PhotoQuality = query.quality_preference.clone().into();
        let include_cover_photo = query.include_cover_photo;
        let album_id_cache = &**album_id_cache;

        async move {
            // Fetch the cover photo if it is requested
            let cover_photo = if include_cover_photo {
                if let Some(id) = &album.cover_photo_id {
                    let photo = Photo::get_by_id(&database, id).await?.unwrap();

                    let quality = if !photo.is_quality_created(qpref.clone()).await? {
                        dal::storage_engine::PhotoQuality::Original
                    } else {
                        qpref
                    };

                    let photo = match storage.engine_type() {
                        EngineType::S3 => match photo.photo_to_proto_url(&storage, quality).await {
                            Ok(v) => v,
                            Err(e) => {
                                return match &e {
                                    DalError::Storage(s) => match s {
                                        StorageEngineError::S3(s) => match s {
                                            S3Error::GetObject(s) => match s {
                                                SdkError::ServiceError(s) => match s.err().kind {
                                                    GetObjectErrorKind::NoSuchKey(_) => {
                                                        album_id_cache.remove(&album.id).await;
                                                        album.delete(&database).await?;
                                                        Ok(None)
                                                    }
                                                    _ => Err(e),
                                                },
                                                _ => Err(e),
                                            },
                                            _ => Err(e),
                                        },
                                        _ => Err(e),
                                    },
                                    _ => Err(e),
                                }
                            }
                        },
                        EngineType::File => photo.photo_to_proto_bytes(&storage, quality).await?,
                    };

                    Some(photo)
                } else {
                    None
                }
            } else {
                None
            };

            Ok(Some(AlbumWithCoverPhoto {
                album: Some(album.to_proto(&database).await?),
                cover_photo,
            }))
        }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, DalError>>()
    .map_err(|e| match e {
        DalError::Storage(e) => Error::from(e),
        DalError::Db(e) => Error::from(e),
    })?
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    Ok(Payload(ListAlbumsResponse { albums }))
}
