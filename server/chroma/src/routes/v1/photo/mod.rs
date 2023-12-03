use crate::routes::appdata::AppData;
use crate::routes::error::{Error, WebResult};
use crate::routes::routable::Routable;
use crate::routes::v1::PhotoQuality;
use actix_web::web;
use actix_web::web::ServiceConfig;
use dal::database::Photo;
use dal::storage_engine::StorageEngineError;
use dal::DalError;
use image::{DynamicImage, ImageOutputFormat};
use proto::photo_respone::Response;
use proto::PhotoRespone;
use serde::Deserialize;
use std::io::Cursor;
use tap::TapFallible;
use tracing::warn;

mod create;
mod delete;
mod get;
mod list;
mod random;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/photo")
                .route("", web::post().to(create::create))
                .route("", web::delete().to(delete::delete))
                .route("", web::get().to(get::get))
                .route("/list", web::get().to(list::list))
                .route("/random", web::get().to(random::random)),
        );
    }
}

#[derive(Eq, PartialEq, Debug, Default, Deserialize)]
pub enum ImageFormat {
    Png,
    Jpeg,
    #[default]
    WebP,
}

/// Retrieve the image from storage and convert it into Protobuf format.
/// Adjusting for the requesting quality and image format.
///
/// # Errors
///
/// - If an IO error occurs
/// - If the storage engine fails
/// - If format conversion fails
async fn get_prepare_image_data(
    data: &AppData,
    photo: Photo<'_>,
    quality: &PhotoQuality,
    format: &ImageFormat,
) -> WebResult<proto::Photo> {
    if format.eq(&ImageFormat::WebP) {
        match photo
            .clone()
            .photo_to_proto_url(&data.storage, quality.clone().into())
            .await
        {
            Ok(p) => return Ok(p),
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
        .photo_to_proto_bytes(&data.storage, quality.clone().into())
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
        response: Some(Response::Bytes(convert_format(bytes, &format)?)),
    });

    Ok(proto)
}

/// Convert an image from one format to the other.
/// The image in `bytes` must be the format specified in `format`.
///
/// # Errors
///
/// If decoding or encoding the image failed.
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

/// Encode a DynamicImage to a specific format.
///
/// ## Parameters
/// - `byte_count` is the number of bytes expected to be in the final image.
///
/// # Errors
///
/// If encoding the image failed.
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

/// Decode raw WebP image bytes to an [image::DynamicImage]
///
/// # Errors
///
/// If the provided bytes don't form a valid WebP image
fn decode_image(bytes: Vec<u8>) -> WebResult<DynamicImage> {
    match webp::Decoder::new(&bytes).decode() {
        Some(webp) => Ok(webp.to_image()),
        None => {
            warn!("Failed to decode WebP image");
            Err(Error::WebpDecode)
        }
    }
}
