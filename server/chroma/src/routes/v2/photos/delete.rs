use actix_web::web;
use dal::database::Photo;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};

/// Delete a photos.
/// If this photos is the cover of it's albums, the albums will no longer have a defined cover image.
/// If this was the last photos in an albums, the albums will *not* be automatically deleted.
///
/// # Errors
///
/// - If the photos does not exist
/// - If something went wrong
pub async fn delete(
    auth: Authorization,
    data: WebData,
    path: web::Path<String>,
) -> WebResult<Empty> {
    let id = path.into_inner();

    if !auth.is_admin
        && !auth
            .has_scope(&data.db, "nl.svsticky.chroma.photos.delete")
            .await?
    {
        return Err(Error::Forbidden);
    }

    Photo::delete(&data.db, &id).await?;

    Ok(Empty)
}
