use actix_multiresponse::Payload;

use dal::database::PhotoQuality;
use dal::database::{Album, Photo};
use proto::DeleteAlbumRequest;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};

/// Delete an existing album.
/// All photos in this album will consequently also be deleted.
///
/// # Errors
///
/// - If the provided `id` does not correspond to any known album
/// - If something went wrong
pub async fn delete(
    auth: Authorization,
    data: WebData,
    payload: Payload<DeleteAlbumRequest>,
) -> WebResult<Empty> {
    if !auth.is_admin
        && !auth
            .has_scope(&data.db, "nl.svsticky.chroma.album.delete")
            .await?
    {
        return Err(Error::Forbidden);
    }

    let album = Album::get_by_id(&data.db, &payload.id)
        .await?
        .ok_or(Error::NotFound)?;

    // Only admins may modify published albums.
    if !album.is_draft && !auth.is_admin {
        return Err(Error::Forbidden);
    }

    let photos = Photo::list_in_album(&data.db, &album.id).await?;
    for photo in photos {
        data.storage
            .delete_photo(&photo.id, &PhotoQuality::Original)
            .await?;
        data.storage
            .delete_photo(&photo.id, &PhotoQuality::W1600)
            .await?;
        data.storage
            .delete_photo(&photo.id, &PhotoQuality::W400)
            .await?;
    }

    album.delete(&data.db).await?;

    Ok(Empty)
}
