use actix_multiresponse::Payload;
use dal::database::Album;
use proto::{CreateAlbumRequest, CreateAlbumResponse};
use crate::routes::appdata::WebData;
use crate::routes::error::{Error, WebResult};

pub async fn create(data: WebData, payload: Payload<CreateAlbumRequest>) -> WebResult<Payload<CreateAlbumResponse>> {
    if payload.name.len() > Album::MAX_NAME_LENGTH {
        return Err(Error::BadRequest(format!("Provided value 'name' with length '{}' exceeds the maximum length of '{}'", payload.name.len(), Album::MAX_NAME_LENGTH)));
    }

    let album = Album::create(&data.db, &payload.name).await?;
    Ok(Payload(CreateAlbumResponse {
        id: album.id
    }))
}
