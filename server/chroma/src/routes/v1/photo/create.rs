use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};
use actix_multiresponse::Payload;
use dal::database::{Album, Photo};
use dal::storage_engine::PhotoQuality;
use image::codecs::png::PngDecoder;
use image::imageops::FilterType;
use image::io::Reader;
use image::{DynamicImage, EncodableLayout, GenericImageView, ImageFormat};
use proto::CreatePhotoRequest;
use std::io::Cursor;
use tap::TapFallible;
use tracing::{info, warn};
use webp::Encoder;

/// Create a new photo in an existing album.
///
/// # Errors
///
/// - If the album does not exist
/// - If something went wrong
pub async fn create(
    auth: Authorization,
    data: WebData,
    payload: Payload<CreatePhotoRequest>,
) -> WebResult<Empty> {
    if !auth.is_admin {
        return Err(Error::Forbidden);
    }

    let album = Album::get_by_id(&data.db, &payload.album_id)
        .await?
        .ok_or(Error::NotFound)?;

    let cursor = Cursor::new(payload.photo_data.clone());
    let image_data = Reader::new(cursor).with_guessed_format().unwrap(); // Cannot fail when using a Cursor

    // Convert to WebP
    let webp_image = match image_data.format() {
        Some(ImageFormat::WebP) => image_data.into_inner().into_inner(),
        Some(ImageFormat::Jpeg | ImageFormat::Png) => {
            let dynamic_image = image_data.decode().map_err(|e| {
                Error::BadRequest(format!(
                    "Failed to decode image. Is the format supported? The error is as follows: {e}"
                ))
            })?;

            convert_image_format(dynamic_image)
                .tap_err(|e| warn!("Failed to encode image: {e}"))
                .map_err(|_| Error::ImageEncoding)?
        }
        _ => {
            return Err(Error::BadRequest(
                "Invalid image or unsupported image provided".into(),
            ))
        }
    };

    // Create the photo metadata in the DB
    let photo = Photo::create(&data.db, &album).await?;

    // Upload the photo to S3
    // If this fails, remove the metadata again
    if let Err(e) = data
        .storage
        .create_photo(&photo.id, webp_image.clone(), PhotoQuality::Original)
        .await
    {
        photo.delete().await?;
        return Err(e.into());
    }

    // Spawn a job to create thumbnails
    let data = data.clone();
    tokio::spawn(async move {
        let id = photo.id;
        let photo = webp_image;

        let cursor = Cursor::new(photo);

        let decoder = match PngDecoder::new(cursor) {
            Ok(d) => d,
            Err(e) => {
                warn!("Failed to create PNG decoder: {e}");
                return;
            }
        };

        let img = match DynamicImage::from_decoder(decoder) {
            Ok(img) => img,
            Err(e) => {
                warn!("Failed to open DynamicImage from decoder: {e}");
                return;
            }
        };

        match convert_quality(&img, 400) {
            Ok(w400) => match data
                .storage
                .create_photo(&id, w400, PhotoQuality::W400)
                .await
            {
                Ok(_) => {}
                Err(e) => warn!("Failed to upload W400 photo: {e}"),
            },
            Err(e) => warn!("Failed to scale to W400: {e}"),
        }

        match convert_quality(&img, 1600) {
            Ok(w1600) => match data
                .storage
                .create_photo(&id, w1600, PhotoQuality::W1600)
                .await
            {
                Ok(_) => {}
                Err(e) => warn!("Failed to upload W1600 photo: {e}"),
            },
            Err(e) => warn!("Failed to scale to W1600: {e}"),
        }
    });

    Ok(Empty)
}

fn convert_image_format(dynamic_image: DynamicImage) -> WebResult<Vec<u8>> {
    let encoder = Encoder::from_image(&dynamic_image).map_err(|_| Error::ImageEncoding)?;

    let encoded_webp = encoder.encode(65.0);

    Ok(encoded_webp.as_bytes().to_vec())
}

fn convert_quality(img: &DynamicImage, target_width: u32) -> color_eyre::Result<Vec<u8>> {
    let (width, height) = img.dimensions();

    info!("Converting {width}x{height} to W{target_width}");

    let target_height = (height as f32 / (width as f32 / target_width as f32)).round() as u32;
    let scaled = if target_width > width {
        img.resize(target_width, target_height, FilterType::Nearest)
    } else {
        img.thumbnail(target_width, target_height)
    };

    Ok(convert_image_format(scaled)?)
}
