use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::WebResult;
use actix_multiresponse::Payload;
use futures::future::join_all;
use dal::database::{Album, DbResult};
use proto::ListAlbumsResponse;

/// List all known albums
///
/// # Errors
///
/// - If something went wrong
pub async fn list(_: Authorization, data: WebData) -> WebResult<Payload<ListAlbumsResponse>> {
    let albums = Album::list(&data.db).await?;
    Ok(Payload(ListAlbumsResponse {
        albums: join_all(albums.into_iter().map(|album| async move {
            album.to_proto().await
        })).await
            .into_iter()
            .collect::<DbResult<Vec<_>>>()?,
    }))
}
