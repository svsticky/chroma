use actix_multiresponse::Payload;

use proto::GetAvailableScopesResponse;

use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};

pub async fn available_scopes(
    auth: Authorization,
) -> WebResult<Payload<GetAvailableScopesResponse>> {
    if !auth.is_admin {
        return Err(Error::Forbidden);
    }

    Ok(Payload(GetAvailableScopesResponse {
        scopes: vec![
            // Albums
            "nl.svsticky.chroma.album.create".into(),
            "nl.svsticky.chroma.album.update".into(),
            "nl.svsticky.chroma.album.delete".into(),
            "nl.svsticky.chroma.album.list.draft".into(),
            // Photos
            "nl.svsticky.chroma.photo.create".into(),
            "nl.svsticky.chroma.photo.delete".into(),
        ],
    }))
}
