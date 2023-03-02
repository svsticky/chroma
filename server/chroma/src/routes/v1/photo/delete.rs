use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};
use actix_multiresponse::Payload;
use dal::database::Photo;
use proto::DeletePhotoRequest;

/// Delete a photo.
/// If this photo is the cover of it's album, the album will no longer have a defined cover image.
/// If this was the last photo in an album, the album will *not* be automatically deleted.
///
/// # Errors
///
/// - If the photo does not exist
/// - If something went wrong
pub async fn delete(
    auth: Authorization,
    data: WebData,
    payload: Payload<DeletePhotoRequest>,
) -> WebResult<Empty> {
    if !auth.is_admin {
        return Err(Error::Forbidden);
    }

    let photo = Photo::get_by_id(&data.db, &payload.photo_id)
        .await?
        .ok_or(Error::NotFound)?;
    let id = photo.id.clone();
    photo.delete().await?;

    data.s3.delete_photo(id).await?;

    Ok(Empty)
}
