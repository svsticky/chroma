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
pub async fn list(auth: Authorization, data: WebData) -> WebResult<Payload<ListAlbumsResponse>> {
    let mut albums = Album::list(&data.db).await?;

    let include_draft = auth.is_admin || auth.has_scope(&data.db, "nl.svsticky.chroma.album.list.draft").await?;
    if !include_draft {
        albums.retain(|f| !f.is_draft);
    }

    Ok(Payload(ListAlbumsResponse {
        albums: join_all(albums.into_iter().map(|album| async move {
            album.to_proto().await
        })).await
            .into_iter()
            .collect::<DbResult<Vec<_>>>()?,
    }))
}
