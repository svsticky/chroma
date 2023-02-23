use actix_multiresponse::Payload;
use dal::database::Album;
use proto::ListAlbumsResponse;
use crate::routes::appdata::WebData;
use crate::routes::error::WebResult;

pub async fn list(data: WebData) -> WebResult<Payload<ListAlbumsResponse>> {
    let albums = Album::list(&data.db).await?;
    Ok(Payload(ListAlbumsResponse {
        albums: albums.into_iter()
            .map(Into::into)
            .collect()
    }))
}