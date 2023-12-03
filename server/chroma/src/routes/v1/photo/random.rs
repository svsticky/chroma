use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use crate::routes::v1::photo::{get_prepare_image_data, ImageFormat};
use crate::routes::v1::PhotoQuality;
use actix_multiresponse::Payload;
use actix_web::http::StatusCode;
use actix_web::web;
use dal::database::Photo;
use proto::GetPhotoResponse;
use rand::Rng;
use serde::Deserialize;
use tap::TapOptional;
use tracing::warn;

#[derive(Debug, Deserialize)]
pub struct Query {
    /// The ID of the previous picture.
    /// Providing this value will ensure the picture returned is
    /// not the same picture as the picture with the ID provided.
    previous_photo_id: Option<String>,
    /// A preference for the quality of the photo.
    /// If the requested quality does not exist, the photo's original resolution will be returned.
    #[serde(default)]
    quality_preference: PhotoQuality,
    /// The format of the image.
    /// E.g., WebP or PNG
    #[serde(default)]
    format: ImageFormat,
}

/// Get a random photo.
///
/// # Errors
///
/// - If something went wrong whilst retrieving or preparing the photo
pub async fn random(
    _: Authorization,
    data: WebData,
    query: web::Query<Query>,
) -> WebResult<Payload<GetPhotoResponse>> {
    let ids = Photo::list_all_ids(&data.db)
        .await?
        .into_iter()
        .filter(|x| match &query.previous_photo_id {
            Some(q) => x.ne(q),
            None => true,
        })
        .collect::<Vec<_>>();

    let mut rng = rand::thread_rng();

    let mut loop_count = 0;
    let photo = loop {
        let id = ids.get(rng.gen_range(0..ids.len()))
            .tap_none(|| warn!("Index out of range: Random index is out of range for list of IDs. This should not be possible."))
            .ok_or(Error::Other(StatusCode::INTERNAL_SERVER_ERROR))?;

        let photo = Photo::get_by_id(&data.db, id).await?;

        match photo {
            Some(p) => break p,
            None => {
                // This loop should really only run once. But it might run twice,
                // but if we loop 5 times, something seriously wrong. Terminate
                // the loop and return an error.
                if loop_count > 5 {
                    return Err(Error::Other(StatusCode::INTERNAL_SERVER_ERROR));
                }

                loop_count += 1;
            }
        }
    };

    let proto =
        get_prepare_image_data(&data, photo, &query.quality_preference, &query.format).await?;
    Ok(Payload(GetPhotoResponse { photo: Some(proto) }))
}
