use actix_multiresponse::Payload;
use proto::CreateAlbumRequest;
use crate::routes::appdata::WebData;
use crate::routes::error::WebResult;

pub async fn create(data: WebData, payload: Payload<CreateAlbumRequest>) -> WebResult<()> {
    todo!()
}
