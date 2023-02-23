use actix_multiresponse::Payload;
use actix_web::web;
use serde::Deserialize;
use dal::database::Album;
use proto::GetAlbumResponse;
use crate::routes::appdata::WebData;
use crate::routes::error::{Error, WebResult};

#[derive(Debug, Deserialize)]
pub struct Query {
    id: String,
}

pub async fn get(data: WebData, query: web::Query<Query>) -> WebResult<Payload<GetAlbumResponse>> {
    let album = Album::get_by_id(&data.db, &query.id)
        .await?
        .ok_or(Error::NotFound)?;
    Ok(Payload(GetAlbumResponse {
        album: Some(album.into()),
        photos: vec![] // TODO
    }))
}