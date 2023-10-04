use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use actix_multiresponse::Payload;
use actix_web::web;
use dal::database::{Album, Photo};
use dal::storage_engine::PhotoQuality;
use futures::future::join_all;
use proto::GetAlbumResponse;
use serde::Deserialize;
use dal::DalError;

#[derive(Debug, Deserialize)]
pub struct Query {
    /// The ID of the album to retrieve
    id: String,
    /// Do not include the pictures of the album in the response.
    without_photos: Option<bool>,
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
    query: web::Query<Query>,
) -> WebResult<Payload<GetAlbumResponse>> {
    let album = Album::get_by_id(&data.db, &query.id)
        .await?
        .ok_or(Error::NotFound)?;

    // If the user requests that photos are not returned, return an empty list.
    let photos = match query.without_photos {
        Some(true) => vec![],
        Some(false) | None => {
            let photos = Photo::list_in_album(&data.db, &album.id).await?;

            // Convert the DAL format to Proto format
            join_all(
                photos
                    .into_iter()
                    .map(|photo| photo.photo_to_proto(&data.storage, PhotoQuality::Original)),
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

    Ok(Payload(GetAlbumResponse {
        photos,
        album: Some(album.to_proto().await?),
    }))
}
