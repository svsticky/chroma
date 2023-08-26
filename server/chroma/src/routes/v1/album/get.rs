use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use actix_multiresponse::Payload;
use actix_web::web;
use dal::database::{Album, Photo};
use dal::storage_engine::StorageEngineError;
use futures::future::join_all;
use proto::GetAlbumResponse;
use serde::Deserialize;
use dal::storage_engine::PhotoQuality;

#[derive(Debug, Deserialize)]
pub struct Query {
    /// The ID of the album to retrieve
    id: String,
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

    let photos = Photo::list_in_album(&data.db, &album.id).await?;
    let photos = join_all(
        photos
            .into_iter()
            .map(|photo| photo.photo_to_proto(&data.storage, PhotoQuality::Original)),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>, StorageEngineError>>()?;

    Ok(Payload(GetAlbumResponse {
        photos,
        album: Some(album.into()),
    }))
}
