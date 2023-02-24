use actix_multiresponse::Payload;
use dal::database::Album;
use proto::DeleteAlbumRequest;
use crate::routes::appdata::WebData;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};

/// Delete an existing album.
/// All photos in this album will consequently also be deleted.
///
/// # Errors
///
/// - If the provided `id` does not correspond to any known album
/// - If something went wrong
pub async fn delete(data: WebData, payload: Payload<DeleteAlbumRequest>) -> WebResult<Empty> {
    Album::get_by_id(&data.db, &payload.id)
        .await?
        .ok_or(Error::NotFound)?
        .delete()
        .await?;
    Ok(Empty)
}