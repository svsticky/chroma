use actix_multiresponse::Payload;
use actix_web::web;

use dal::database::Photo;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};

/// Retrieve a photos by its ID.
///
/// # Errors
///
/// - If the photos does not exist
/// - If something went wrong
pub async fn get(
    _: Authorization,
    data: WebData,
    path: web::Path<String>,
) -> WebResult<Payload<proto::Photo>> {
    let id = path.into_inner();

    Ok(Payload(
        Photo::get(&data.db, &id)
            .await?
            .ok_or(Error::NotFound)?
            .into(),
    ))
}
