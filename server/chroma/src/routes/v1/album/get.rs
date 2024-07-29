use actix_multiresponse::Payload;
use actix_web::web;
use futures::future::join_all;
use serde::Deserialize;

use dal::DalError;
use dal::database::{Album, Photo};
use dal::database::PhotoQuality;
use proto::{AlbumWithCoverPhoto, GetAlbumResponse};

use crate::routes::appdata::{AlbumIdCache, WebData};
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};

#[derive(Debug, Deserialize)]
pub struct Query {
    /// The ID of the album to retrieve
    id: String,
    /// Do not include the pictures of the album in the response.
    without_photos: Option<bool>,

    include_cover_photo: Option<bool>,
}

/// Retrieve an album and all its photos by its ID.
///
/// # Errors
///
/// - If the requested album does not exist
/// - If something went wrong
pub async fn get(
    _: Authorization,
    data: WebData,
    album_id_cache: web::Data<AlbumIdCache>,
    query: web::Query<Query>,
) -> WebResult<Payload<GetAlbumResponse>> {
    let album = match album_id_cache.get(&query.id).await {
        Some(v) => v,
        None => Album::get_by_id(&data.db, &query.id)
            .await?
            .ok_or(Error::NotFound)?,
    };

    // If the user requests that photos are not returned, return an empty list.
    let photos = match query.without_photos {
        Some(true) => vec![],
        Some(false) | None => {
            let photos = Photo::list_in_album(&data.db, &album.id).await?;

            // Convert the DAL format to Proto format
            join_all(
                photos
                    .into_iter()
                    .map(|photo| photo.photo_to_proto_bytes(&data.storage, PhotoQuality::Original)),
            )
            .await
            .into_iter()
            .collect::<Result<Vec<_>, DalError>>()
            .map_err(|e| match e {
                DalError::Db(e) => Error::from(e),
                DalError::Storage(e) => Error::from(e),
            })?
        }
    };

    let cover_photo = if query.include_cover_photo.unwrap_or(true) {
        if let Some(id) = &album.cover_photo_id {
            let photo = Photo::get_by_id(&data.db, id)
                .await?
                .ok_or(Error::NotFound)?;

            let photo = photo
                .photo_to_proto_url(&data.storage, &PhotoQuality::W400)
                .await
                .map_err(|e| match e {
                    DalError::Storage(e) => Error::from(e),
                    DalError::Db(e) => Error::from(e),
                })?;

            Some(photo)
        } else {
            None
        }
    } else {
        None
    };

    Ok(Payload(GetAlbumResponse {
        photos,
        album: Some(AlbumWithCoverPhoto {
            album: Some(album.to_proto(&data.db).await?),
            cover_photo,
        }),
    }))
}
