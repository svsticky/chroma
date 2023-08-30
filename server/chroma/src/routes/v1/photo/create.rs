use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use actix_multiresponse::Payload;
use dal::database::{Album, Photo};
use dal::storage_engine::PhotoQuality;
use image::codecs::png::PngDecoder;
use image::imageops::FilterType;
use image::io::Reader;
use image::{DynamicImage, GenericImageView, ImageFormat, ImageOutputFormat};
use proto::{CreatePhotoRequest, CreatePhotoResponse};
use std::io::Cursor;
use tracing::{info, warn};

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
) -> WebResult<Payload<CreatePhotoResponse>> {
    if !auth.is_admin {
        return Err(Error::Forbidden);
    }

    let album = Album::get_by_id(&data.db, &payload.album_id)
        .await?
        .ok_or(Error::NotFound)?;

    let photo_size = payload.photo_data.len();
    let cursor = Cursor::new(payload.photo_data.clone());
    let image = Reader::new(cursor).with_guessed_format().unwrap(); // Cannot fail when using a Cursor

    // Convert to PNG if the current format is JPEG
    let png_image = match image.format() {
        Some(ImageFormat::Png) => image.into_inner().into_inner(),
        Some(ImageFormat::Jpeg) => {
            let decoded = image.decode()
                .map_err(|e| Error::BadRequest(format!("Failed to decode image. Is the format PNG or JPEG? The error is as follows: {e}")))?;

            let mut cursor = Cursor::new(Vec::with_capacity(photo_size));
            decoded
                .write_to(&mut cursor, ImageOutputFormat::Png)
                .map_err(|_| Error::ImageEncoding)?;

            cursor.into_inner()
        }
        _ => {
            return Err(Error::BadRequest(
                "Invalid image or non JPEG/PNG image provided".into(),
            ))
        }
    };

    // Create the photo metadata in the DB
    let photo = Photo::create(&data.db, &album).await?;

    // Upload the photo to S3
    // If this fails, remove the metadata again
    if let Err(e) = data
        .storage
        .create_photo(&photo.id, png_image.clone(), PhotoQuality::Original)
        .await
    {
        photo.delete().await?;
        return Err(e.into());
    }

    // Spawn a job to create thumbnails
    let data = data.clone();
    // Clone the ID for the async job to use
    let photo_id = photo.id.clone();
    tokio::spawn(async move {
        let id = photo_id;
        let photo = png_image;

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

    Ok(Payload(CreatePhotoResponse { photo_id: photo.id }))
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

    let mut cursor = Cursor::new(Vec::with_capacity((target_width * target_height) as usize));
    scaled
        .write_to(&mut cursor, ImageOutputFormat::Png)
        .map_err(|_| Error::ImageEncoding)?;

    Ok(cursor.into_inner())
}
