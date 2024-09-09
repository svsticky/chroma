use actix_multiresponse::Payload;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::WebResult;
use dal::database::Photo;
use proto::{BatchGetPhotosRequest, BatchGetPhotosResponse};

/// Retrieve a photos by its ID.
///
/// # Errors
///
/// - If the photos does not exist
/// - If something went wrong
pub async fn batch_get(
    _: Authorization,
    data: WebData,
    payload: Payload<BatchGetPhotosRequest>,
) -> WebResult<Payload<BatchGetPhotosResponse>> {
    let photos = if !payload.ids.is_empty() {
        Photo::get_many(&data.db, &payload.ids).await?
    } else {
        vec![]
    };

    Ok(Payload(BatchGetPhotosResponse {
        photos: photos.into_iter().map(|p| p.into()).collect(),
    }))
}
