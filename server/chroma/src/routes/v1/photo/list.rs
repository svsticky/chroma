use actix_multiresponse::Payload;
use actix_web::web;
use futures::future::join_all;
use serde::Deserialize;

use dal::DalError;
use dal::database::Photo;
use proto::ListPhotoResponse;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use crate::routes::v1::PhotoQuality;

#[derive(Debug, Deserialize)]
pub struct Query {
    /// The ID of the album to list all photos from
    album_id: Option<String>,
    /// A preference for the quality of a photo.
    /// If the requested quality does not exist, the photo's original resolution will be returned.
    #[serde(default)]
    quality_preference: PhotoQuality,
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

    let photos = join_all(photos.into_iter().map(|p| {
        let storage = data.storage.clone();
        let qpref: dal::database::PhotoQuality = query.quality_preference.clone().into();

        async move { p.photo_to_proto_url(&storage, &qpref).await }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, DalError>>()
    .map_err(|e| match e {
        DalError::Storage(e) => Error::from(e),
        DalError::Db(e) => Error::from(e),
    })?;

    Ok(Payload(ListPhotoResponse { photos }))
}
