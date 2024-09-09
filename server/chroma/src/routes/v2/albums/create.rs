use actix_multiresponse::Payload;

use dal::database::Album;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};

/// Create a new empty albums.
/// The albums will not contain any photos yet.
///
/// # Errors
///
/// - If the provided `name`'s length is longer than [Album::MAX_NAME_LENGTH]
/// - If something went wrong
pub async fn create(
    auth: Authorization,
    data: WebData,
    payload: Payload<proto::Album>,
) -> WebResult<Payload<proto::Album>> {
    if !auth.is_admin {
        let published = &payload.published.unwrap_or(false);
        let create_scope = auth
            .has_scope(&data.db, "nl.svsticky.chroma.albums.create")
            .await?;

        if *published && !create_scope {
            return Err(Error::Forbidden);
        }
    }

    if let Some(name) = &payload.name {
        if name.len() > Album::MAX_NAME_LENGTH {
            return Err(Error::BadRequest(format!(
                "Provided value 'name' with length '{}' exceeds the maximum length of '{}'",
                name.len(),
                Album::MAX_NAME_LENGTH
            )));
        }
    }

    Ok(Payload(
        Album::create(
            &data.db,
            payload.name.clone().unwrap_or_default(),
            payload.published.unwrap_or(false),
            auth.to_dal_user_type(&data.db).await?,
        )
        .await?
        .into(),
    ))
}
