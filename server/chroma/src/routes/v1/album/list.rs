use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::WebResult;
use actix_multiresponse::Payload;
use dal::database::Album;
use proto::ListAlbumsResponse;

/// List all known albums
///
/// # Errors
///
/// - If something went wrong
pub async fn list(_: Authorization, data: WebData) -> WebResult<Payload<ListAlbumsResponse>> {
    let albums = Album::list(&data.db).await?;
    Ok(Payload(ListAlbumsResponse {
        albums: albums.into_iter().map(Into::into).collect(),
    }))
}
