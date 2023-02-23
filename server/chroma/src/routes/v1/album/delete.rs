use actix_multiresponse::Payload;
use dal::database::Album;
use proto::DeleteAlbumRequest;
use crate::routes::appdata::WebData;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};

pub async fn delete(data: WebData, payload: Payload<DeleteAlbumRequest>) -> WebResult<Empty> {
    Album::get_by_id(&data.db, &payload.id)
        .await?
        .ok_or(Error::NotFound)?
        .delete()
        .await?;
    Ok(Empty)
}