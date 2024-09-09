use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};
use actix_multiresponse::Payload;
use dal::database::Photo;
use proto::BatchDeletePhotosRequest;

/// Delete a photos.
/// If this photos is the cover of it's albums, the albums will no longer have a defined cover image.
/// If this was the last photos in an albums, the albums will *not* be automatically deleted.
///
/// # Errors
///
/// - If the photos does not exist
/// - If something went wrong
pub async fn batch_delete(
    auth: Authorization,
    data: WebData,
    payload: Payload<BatchDeletePhotosRequest>,
) -> WebResult<Empty> {
    if !auth.is_admin
        && !auth
            .has_scope(&data.db, "nl.svsticky.chroma.photos.delete")
            .await?
    {
        return Err(Error::Forbidden);
    }

    if !payload.ids.is_empty() {
        Photo::delete_many(&data.db, &payload.ids).await?;
    }

    Ok(Empty)
}
