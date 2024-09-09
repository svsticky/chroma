use actix_multiresponse::Payload;
use actix_web::web;

use dal::database::Album;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};

/// Retrieve an albums and all its photos by its ID.
///
/// # Errors
///
/// - If the requested albums does not exist
/// - If something went wrong
pub async fn get(
    _: Authorization,
    data: WebData,
    path: web::Path<String>,
) -> WebResult<Payload<proto::Album>> {
    let id = path.into_inner();

    Ok(Payload(
        Album::get_by_id(&data.db, &id)
            .await?
            .ok_or(Error::NotFound)?
            .into(),
    ))
}
