use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};
use actix_multiresponse::Payload;
use dal::database::PhotoQuality;
use dal::database::{Album, Photo};
use proto::DeletePhotoRequest;
use reqwest::StatusCode;

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
    if !auth.is_admin
        && !auth
            .has_scope(&data.db, "nl.svsticky.chroma.photo.delete")
            .await?
    {
        return Err(Error::Forbidden);
    }

    let photo = Photo::get_by_id(&data.db, &payload.photo_id)
        .await?
        .ok_or(Error::NotFound)?;

    if !auth.is_admin {
        let album = Album::get_by_id(&data.db, &photo.album_id)
            .await?
            .ok_or(Error::Other(StatusCode::INTERNAL_SERVER_ERROR))?;

        // Only admins may modify published albums
        if !album.is_draft {
            return Err(Error::Forbidden);
        }
    }

    let id = photo.id.clone();
    photo.delete().await?;

    data.storage
        .delete_photo(&id, &PhotoQuality::Original)
        .await?;
    data.storage.delete_photo(&id, &PhotoQuality::W1600).await?;
    data.storage.delete_photo(&id, &PhotoQuality::W400).await?;

    Ok(Empty)
}
