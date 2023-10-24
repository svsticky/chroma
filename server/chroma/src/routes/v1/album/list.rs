use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use crate::routes::v1::PhotoQuality;
use actix_multiresponse::Payload;
use actix_web::web;
use dal::database::{Album, Photo};
use dal::storage_engine::EngineType;
use dal::DalError;
use futures::future::join_all;
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
    data: WebData,
    query: web::Query<Query>,
) -> WebResult<Payload<ListAlbumsResponse>> {
    let mut albums = Album::list(&data.db).await?;

    let include_draft = auth.is_admin
        || auth
            .has_scope(&data.db, "nl.svsticky.chroma.album.list.draft")
            .await?;

    if !include_draft {
        albums.retain(|f| !f.is_draft);
    }

    let albums = join_all(albums.into_iter().map(|album| {
        let storage = data.storage.clone();
        let database = data.db.clone();
        let qpref = query.quality_preference.clone().into();
        let include_cover_photo = query.include_cover_photo;

        async move {
            let cover_photo = if include_cover_photo {
                if let Some(id) = &album.cover_photo_id {
                    let photo = Photo::get_by_id(&database, id).await?.unwrap();

                    let photo = match storage.engine_type() {
                        EngineType::S3 => photo.photo_to_proto_url(&storage, qpref).await?,
                        EngineType::File => photo.photo_to_proto_bytes(&storage, qpref).await?,
                    };

                    Some(photo)
                } else {
                    None
                }
            } else {
                None
            };

            Ok(AlbumWithCoverPhoto {
                album: Some(album.to_proto().await?),
                cover_photo,
            })
        }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, DalError>>()
    .map_err(|e| match e {
        DalError::Storage(e) => Error::from(e),
        DalError::Db(e) => Error::from(e),
    })?;

    Ok(Payload(ListAlbumsResponse { albums }))
}
