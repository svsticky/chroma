use actix_multiresponse::Payload;

use dal::database::Album;
use proto::ListAlbumsResponse;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::WebResult;

/// List all known albums
///
/// # Errors
///
/// - If something went wrong
pub async fn list(auth: Authorization, data: WebData) -> WebResult<Payload<ListAlbumsResponse>> {
    // Check if we should include draft albums
    let include_draft = auth.is_admin
        || auth
            .has_scope(&data.db, "nl.svsticky.chroma.albums.list.draft")
            .await?;

    let albums = Album::list(&data.db, !include_draft).await?;

    Ok(Payload(ListAlbumsResponse {
        albums: albums.into_iter().map(|album| album.into()).collect(),
    }))
}
