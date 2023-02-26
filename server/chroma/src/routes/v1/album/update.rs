use actix_multiresponse::Payload;
use dal::database::Album;
use proto::UpdateAlbumRequest;
use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};

/// Update the metadata of an existing album.
/// Currently, only the name can be updated.
///
/// # Errors
///
/// - If the new name's length is longer than [Album::MAX_NAME_LENGTH]
/// - If the album to be updated could not be found
/// - If something went wrong
pub async fn update(auth: Authorization, data: WebData, payload: Payload<UpdateAlbumRequest>) -> WebResult<Empty> {
    if !auth.is_admin {
        return Err(Error::Forbidden);
    }

    if payload.name.len() > Album::MAX_NAME_LENGTH {
        return Err(Error::BadRequest(format!("Provided value 'name' with length '{}' exceeds the maximum length of '{}'", payload.name.len(), Album::MAX_NAME_LENGTH)));
    }

    Album::get_by_id(&data.db, &payload.id)
        .await?
        .ok_or(Error::NotFound)?
        .update_name(&payload.name).await?;

    Ok(Empty)
}