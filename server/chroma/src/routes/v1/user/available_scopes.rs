use actix_multiresponse::Payload;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use proto::GetAvailableScopesResponse;

pub async fn available_scopes(auth: Authorization) -> WebResult<Payload<GetAvailableScopesResponse>> {
    if !auth.is_admin {
        return Err(Error::Forbidden);
    }

    Ok(Payload(GetAvailableScopesResponse {
        scopes: vec![
            // Albums
            "nl.svsticky.album.create".into(),
            "nl.svsticky.album.update".into(),
            "nl.svsticky.album.delete".into(),
            // Photos
            "nl.svsticky.photo.create".into(),
            "nl.svsticky.photo.delete".into(),
        ]
    }))
}