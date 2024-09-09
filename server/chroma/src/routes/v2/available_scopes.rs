use actix_multiresponse::Payload;

use proto::GetAvailableScopesResponse;

use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};

#[allow(dead_code)]
pub async fn available_scopes(
    auth: Authorization,
) -> WebResult<Payload<GetAvailableScopesResponse>> {
    if !auth.is_admin {
        return Err(Error::Forbidden);
    }

    Ok(Payload(GetAvailableScopesResponse {
        scopes: vec![
            // Albums
            "nl.svsticky.chroma.albums.create".into(),
            "nl.svsticky.chroma.albums.update".into(),
            "nl.svsticky.chroma.albums.delete".into(),
            "nl.svsticky.chroma.albums.list.draft".into(),
            // Photos
            "nl.svsticky.chroma.photos.create".into(),
            "nl.svsticky.chroma.photos.delete".into(),
        ],
    }))
}
