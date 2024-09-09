use actix_web::web;

use dal::database::Album;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};

/// Delete an existing albums.
/// All photos in this albums will consequently also be deleted.
///
/// # Errors
///
/// - If the provided `id` does not correspond to any known albums
/// - If something went wrong
pub async fn delete(
    auth: Authorization,
    data: WebData,
    path: web::Path<String>,
) -> WebResult<Empty> {
    let id = path.into_inner();

    if !auth.is_admin
        && !auth
            .has_scope(&data.db, "nl.svsticky.chroma.albums.delete")
            .await?
    {
        return Err(Error::Forbidden);
    }

    let album = Album::get_by_id(&data.db, &id)
        .await?
        .ok_or(Error::NotFound)?;

    // Only admins may modify published albums.
    if !album.published && !auth.is_admin {
        return Err(Error::Forbidden);
    }

    album.delete(&data.db).await?;

    Ok(Empty)
}
