use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use crate::routes::v1::PhotoQuality;
use actix_multiresponse::Payload;
use actix_web::web;
use dal::database::Photo;
use proto::GetPhotoResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Query {
    /// The ID of the photo to retrieve
    id: String,
    /// A preference for the quality of a photo.
    /// If the requested quality does not exist, the photo's original resolution will be returned.
    #[serde(default)]
    quality_preference: PhotoQuality,
}

/// Retrieve a photo by its ID.
///
/// # Errors
///
/// - If the photo does not exist
/// - If something went wrong
pub async fn get(
    _: Authorization,
    data: WebData,
    query: web::Query<Query>,
) -> WebResult<Payload<GetPhotoResponse>> {
    let photo = Photo::get_by_id(&data.db, &query.id)
        .await?
        .ok_or(Error::NotFound)?;

    Ok(Payload(GetPhotoResponse {
        photo: Some(
            photo
                .photo_to_proto(&data.storage, query.quality_preference.clone().into())
                .await?,
        ),
    }))
}
