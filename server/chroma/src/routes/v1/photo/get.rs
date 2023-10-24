use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use crate::routes::v1::PhotoQuality;
use actix_multiresponse::Payload;
use actix_web::web;
use dal::database::Photo;
use dal::storage_engine::StorageEngineError;
use dal::DalError;
use image::{DynamicImage, ImageOutputFormat};
use proto::{GetPhotoResponse, PhotoRespone, PhotoResponseType};
use serde::Deserialize;
use std::io::Cursor;
use tap::TapFallible;
use tracing::warn;
use proto::photo_respone::Response;

#[derive(Debug, Deserialize)]
pub struct Query {
    /// The ID of the photo to retrieve
    id: String,
    /// A preference for the quality of a photo.
    /// If the requested quality does not exist, the photo's original resolution will be returned.
    #[serde(default)]
    quality_preference: PhotoQuality,
    /// The format of the image.
    /// E.g., WebP or PNG
    #[serde(default)]
    format: ImageFormat,
}

#[derive(Eq, PartialEq, Debug, Default, Deserialize)]
pub enum ImageFormat {
    Png,
    Jpeg,
    #[default]
    WebP,
}

/// Retrieve a photo by its ID.
///
/// # Errors
///
/// - If the photo does not exist
/// - If something went wrong
pub async fn get(
    _: Authorization,
    data: WebData,
    query: web::Query<Query>,
) -> WebResult<Payload<GetPhotoResponse>> {
    let photo = Photo::get_by_id(&data.db, &query.id)
        .await?
        .ok_or(Error::NotFound)?;

    if query.format.eq(&ImageFormat::WebP) {
        match photo
            .photo_to_url(&data.storage, query.quality_preference.clone().into())
            .await
        {
            Ok(p) => {
                return Ok(Payload(GetPhotoResponse {
                    response_type: PhotoResponseType::Url as i32,
                    response: Some(PhotoRespone {
                        response: Some(Response::Url(p))
                    }),
                }));
            }
            Err(e) => match e {
                DalError::Storage(e) => match e {
                    // URL mode is not supported
                    StorageEngineError::NotSupported => {}
                    _ => return Err(e.into()),
                },
                DalError::Db(e) => return Err(e.into()),
            },
        }
    }

    let mut proto = photo
        .photo_to_proto(&data.storage, query.quality_preference.clone().into())
        .await
        .map_err(|e| match e {
            DalError::Storage(e) => Error::from(e),
            DalError::Db(e) => Error::from(e),
        })?;

    proto.photo_data = convert_format(proto.photo_data, &query.format)?;

    Ok(Payload(GetPhotoResponse {
        response_type: PhotoResponseType::InResponse as i32,
        response: Some(PhotoRespone {
            response: Some(Response::Photo(proto))
        }),
    }))
}

fn convert_format(bytes: Vec<u8>, format: &ImageFormat) -> WebResult<Vec<u8>> {
    match format {
        ImageFormat::WebP => Ok(bytes),
        ImageFormat::Png => {
            let byte_count = bytes.len();
            reencode_dynamic_image(decode_image(bytes)?, ImageOutputFormat::Png, byte_count)
        }
        ImageFormat::Jpeg => {
            let byte_count = bytes.len();
            reencode_dynamic_image(
                decode_image(bytes)?,
                ImageOutputFormat::Jpeg(100),
                byte_count,
            )
        }
    }
}

fn reencode_dynamic_image(
    image: DynamicImage,
    format: ImageOutputFormat,
    byte_count: usize,
) -> WebResult<Vec<u8>> {
    let mut cursor = Cursor::new(Vec::with_capacity(byte_count));
    image
        .write_to(&mut cursor, format)
        .tap_err(|e| warn!("Failed to write image in format: {e}"))?;

    Ok(cursor.into_inner())
}

fn decode_image(bytes: Vec<u8>) -> WebResult<DynamicImage> {
    match webp::Decoder::new(&bytes).decode() {
        Some(webp) => Ok(webp.to_image()),
        None => {
            warn!("Failed to decode WebP image");
            Err(Error::WebpDecode)
        }
    }
}
