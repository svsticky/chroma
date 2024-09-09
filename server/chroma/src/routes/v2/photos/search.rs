use actix_multiresponse::Payload;
use dal::database::Photo;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::WebResult;

/// List all photos, either all known or all from one albums.
/// If the `album_id` provided does not correspond to any known albums,
/// an empty set will be returned.
///
/// # Errors
///
/// - If something went wrong
pub async fn search(
    _: Authorization,
    data: WebData,
    payload: Payload<proto::SearchPhotosRequest>,
) -> WebResult<Payload<proto::SearchPhotosResponse>> {
    let photos = Photo::search(&data.db, &payload.album_id).await?;

    Ok(Payload(proto::SearchPhotosResponse {
        photos: photos.into_iter().map(|p| p.into()).collect(),
    }))
}
