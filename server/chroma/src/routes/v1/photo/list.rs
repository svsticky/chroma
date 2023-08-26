use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::WebResult;
use actix_multiresponse::Payload;
use actix_web::web;
use dal::database::Photo;
use dal::s3::{PhotoQuality, S3Error};
use futures::future::join_all;
use proto::ListPhotoResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Query {
    /// The ID of the album to list all photos from
    album_id: Option<String>,
    #[serde(default)]
    quality_preference: Quality
}

#[derive(Debug, Default, Deserialize)]
pub enum Quality {
    #[default]
    Original,
    W400,
    W1600,
}

/// List all photos, either all known or all from one album.
/// If the `album_id` provided does not correspond to any known album,
/// an empty set will be returned.
///
/// # Errors
///
/// - If something went wrong
pub async fn list(
    _: Authorization,
    data: WebData,
    query: web::Query<Query>,
) -> WebResult<Payload<ListPhotoResponse>> {
    let photos = if let Some(album_id) = &query.album_id {
        Photo::list_in_album(&data.db, album_id).await?
    } else {
        Photo::list(&data.db).await?
    };

    let quality_preference = match query.quality_preference {
        Quality::Original => PhotoQuality::Original,
        Quality::W1600 => PhotoQuality::W1600,
        Quality::W400 => PhotoQuality::W400,
    };

    Ok(Payload(ListPhotoResponse {
        photos: join_all(
            photos
                .into_iter()
                .map(|photo| photo.photo_to_proto(&data.s3, quality_preference.clone())),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<_>, S3Error>>()?,
    }))
}
