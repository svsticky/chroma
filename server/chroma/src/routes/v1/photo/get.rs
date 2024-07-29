use std::io::Cursor;

use actix_multiresponse::Payload;
use actix_web::web;
use image::{DynamicImage, ImageOutputFormat};
use serde::Deserialize;
use tap::TapFallible;
use tracing::warn;

use dal::DalError;
use dal::database::Photo;
use proto::{GetPhotoResponse, PhotoRespone};
use proto::photo_respone::Response;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use crate::routes::v1::PhotoQuality;

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
    /// By the default the server may choose to return the image bytes or return a signed S3 URL.
    /// By setting this to true the service will return the image bytes.
    /// Defaults to false.
    #[serde(default)]
    force_bytes: bool,
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

    if query.format.eq(&ImageFormat::WebP) && !query.force_bytes {
        return match photo
            .clone()
            .photo_to_proto_url(&data.storage, &query.quality_preference.clone().into())
            .await
        {
            Ok(p) => Ok(Payload(GetPhotoResponse { photo: Some(p) })),
            Err(e) => match e {
                DalError::Storage(e) => Err(e.into()),
                DalError::Db(e) => Err(e.into()),
            },
        };
    }

    let mut proto = photo
        .photo_to_proto_bytes(&data.storage, query.quality_preference.clone().into())
        .await
        .map_err(|e| match e {
            DalError::Storage(e) => Error::from(e),
            DalError::Db(e) => Error::from(e),
        })?;

    let bytes = if let Response::Bytes(b) = proto.data.unwrap().response.unwrap() {
        b
    } else {
        unreachable!()
    };
    proto.data = Some(PhotoRespone {
        response: Some(Response::Bytes(convert_format(bytes, &query.format)?)),
    });

    Ok(Payload(GetPhotoResponse { photo: Some(proto) }))
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
