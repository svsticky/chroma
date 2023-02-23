use actix_multiresponse::Payload;
use dal::database::Album;
use proto::UpdateAlbumRequest;
use crate::routes::appdata::WebData;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};

pub async fn update(data: WebData, payload: Payload<UpdateAlbumRequest>) -> WebResult<Empty> {
    if payload.name.len() > Album::MAX_NAME_LENGTH {
        return Err(Error::BadRequest(format!("Provided value 'name' with length '{}' exceeds the maximum length of '{}'", payload.name.len(), Album::MAX_NAME_LENGTH)));
    }

    Album::get_by_id(&data.db, &payload.id)
        .await?
        .ok_or(Error::NotFound)?
        .update_name(&payload.name).await?;

    Ok(Empty)
}