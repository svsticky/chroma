use actix_multiresponse::Payload;
use actix_web::web;
use futures::future::join_all;
use serde::Deserialize;
use dal::database::Photo;
use proto::ListPhotoResponse;
use crate::routes::appdata::WebData;
use crate::routes::error::WebResult;
use crate::routes::v1::photo::photo_to_proto;

#[derive(Debug, Deserialize)]
pub struct Query {
    /// The ID of the album to list all photos from
    album_id: Option<String>
}

/// List all photos, either all known or all from one album.
/// If the `album_id` provided does not correspond to any known album,
/// an empty set will be returned.
///
/// # Errors
///
/// - If something went wrong
pub async fn list(data: WebData, query: web::Query<Query>) -> WebResult<Payload<ListPhotoResponse>> {
    let photos = if let Some(album_id) = &query.album_id {
        Photo::list_in_album(&data.db, album_id).await?
    } else {
        Photo::list(&data.db).await?
    };

    Ok(Payload(ListPhotoResponse {
        photos: join_all(photos.into_iter()
            .map(|photo| photo_to_proto(&data.s3, photo)))
            .await
            .into_iter()
            .collect::<WebResult<Vec<_>>>()?
    }))
}