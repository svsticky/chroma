use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use actix_multiresponse::Payload;
use tracing::trace;
use dal::database::Album;
use proto::{CreateAlbumRequest, CreateAlbumResponse};

/// Create a new empty album.
/// The album will not contain any photos yet.
///
/// # Errors
///
/// - If the provided `name`'s length is longer than [Album::MAX_NAME_LENGTH]
/// - If something went wrong
pub async fn create(
    auth: Authorization,
    data: WebData,
    payload: Payload<CreateAlbumRequest>,
) -> WebResult<Payload<CreateAlbumResponse>> {
    if !auth.is_admin {
        let is_draft = payload.is_draft.unwrap_or(false);
        let create_scope = auth.has_scope(&data.db, "nl.svsticky.chroma.album.create").await?;
        trace!("is_draft: {is_draft}");
        trace!("has_scope: {create_scope}");
        if !is_draft && !create_scope {
            return Err(Error::Forbidden);
        }
    }

    if payload.name.len() > Album::MAX_NAME_LENGTH {
        return Err(Error::BadRequest(format!(
            "Provided value 'name' with length '{}' exceeds the maximum length of '{}'",
            payload.name.len(),
            Album::MAX_NAME_LENGTH
        )));
    }

    let album = Album::create(
        &data.db,
        &payload.name,
        payload.is_draft.unwrap_or(false),
        auth.to_dal_user_type(&data.db).await?,

    ).await?;
    Ok(Payload(CreateAlbumResponse { id: album.id }))
}
