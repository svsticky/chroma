use actix_multiresponse::Payload;
use actix_web::web;
use serde::Deserialize;
use dal::database::Photo;
use proto::GetPhotoResponse;
use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};

#[derive(Debug, Deserialize)]
pub struct Query {
    /// The ID of the photo to retrieve
    id: String,
}

/// Retrieve a photo by its ID.
///
/// # Errors
///
/// - If the photo does not exist
/// - If something went wrong
pub async fn get(_: Authorization, data: WebData, query: web::Query<Query>) -> WebResult<Payload<GetPhotoResponse>> {
    let photo = Photo::get_by_id(&data.db, &query.id)
        .await?
        .ok_or(Error::NotFound)?;

    Ok(Payload(GetPhotoResponse {
        photo: Some(photo.photo_to_proto(&data.s3).await?)
    }))
}
