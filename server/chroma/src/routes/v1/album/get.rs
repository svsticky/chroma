use actix_multiresponse::Payload;
use actix_web::web;
use futures::future::join_all;
use serde::Deserialize;
use dal::database::{Album, Photo};
use dal::s3::S3Error;
use proto::GetAlbumResponse;
use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};

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
pub async fn get(_: Authorization, data: WebData, query: web::Query<Query>) -> WebResult<Payload<GetAlbumResponse>> {
    let album = Album::get_by_id(&data.db, &query.id)
        .await?
        .ok_or(Error::NotFound)?;

    let photos = Photo::list_in_album(&data.db, &album.id).await?;
    let photos = join_all(photos.into_iter().map(|photo| photo.photo_to_proto(&data.s3))).await
        .into_iter()
        .collect::<Result<Vec<_>, S3Error>>()?;

    Ok(Payload(GetAlbumResponse {
        photos,
        album: Some(album.into()),
    }))
}