use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use actix_multiresponse::Payload;
use dal::database::{Album, Photo};
use dal::storage_engine::PhotoQuality;
use image::imageops::FilterType;
use image::io::Reader;
use image::{DynamicImage, GenericImageView};
use img_parts::{Bytes, DynImage, ImageEXIF};
use proto::{CreatePhotoRequest, CreatePhotoResponse};
use std::io::Cursor;
use std::str::FromStr;
use exif::{In, Tag};
use tap::TapFallible;
use time::OffsetDateTime;
use tracing::{debug, warn};
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
) -> WebResult<Payload<CreatePhotoResponse>> {
    if !auth.is_admin {
        if !auth
            .has_scope(&data.db, "nl.svsticky.chroma.photo.create")
            .await?
        {
            return Err(Error::Forbidden);
        }
    }

    let album = Album::get_by_id(&data.db, &payload.album_id)
        .await?
        .ok_or(Error::NotFound)?;

    // Album must be un-published for non-admins to modify them.
    if !album.is_draft && !auth.is_admin {
        return Err(Error::Forbidden);
    }

    let cursor = Cursor::new(payload.photo_data.clone());
    let image_data = Reader::new(cursor.clone()).with_guessed_format().unwrap(); // Cannot fail when using a Cursor

    // Decode to DynamicImage
    let dynamic_image = image_data.decode()
        .map_err(|e| Error::BadRequest(format!(
            "Failed to decode image. Is the image format supported? The error is as follows: {e}"
        )))?;

    // Re-Encode to WebP
    let webp_image = Encoder::from_image(&dynamic_image)
        .tap_err(|e| warn!("Failed to create image encoder: {e}"))
        .map_err(|_| Error::ImageEncoding)?
        .encode(100.0);

    // Parse EXIF timestamp, if available
    let timestamp = try_parse_exif_timestamp(webp_image.to_vec())
        .unwrap_or(OffsetDateTime::now_utc().unix_timestamp());

    // Strip EXIF metadata
    let data_stripped = strip_exif_metadata(webp_image.to_vec())?;

    // Create the photo metadata
    let photo_metadata = Photo::create(&data.db, &album, timestamp).await?;

    // Upload original quality image
    let photo_id = photo_metadata.id.clone();
    let storage = data.storage.clone();
    let data_original = data_stripped.clone();
    tokio::spawn(async move {
        let _ = storage
            .create_photo(photo_id, data_original.clone(), PhotoQuality::Original)
            .await;
    });

    // Decode WebP image to DynamicImage again
    let dynamic_image = webp::Decoder::new(&data_stripped)
        .decode()
        .unwrap() // It is guaranteed to be a WebP image
        .to_image();

    // W400 quality
    let photo_id = photo_metadata.id.clone();
    let storage = data.storage.clone();
    let image = dynamic_image.clone();
    tokio::spawn(async move {
        match convert_quality(&image, 400) {
            Ok(w400) => match storage
                .create_photo(&photo_id, w400, PhotoQuality::W400)
                .await
            {
                Ok(_) => {}
                Err(e) => warn!("Failed to upload W400 photo: {e}"),
            },
            Err(e) => warn!("Failed to scale to W400: {e}"),
        }
    });

    let photo_id_for_response = photo_metadata.id.clone();

    // W1600 quality
    let photo_id = photo_metadata.id.clone();
    tokio::spawn(async move {
        match convert_quality(&dynamic_image, 1600) {
            Ok(w1600) => match data
                .storage
                .create_photo(&photo_id, w1600, PhotoQuality::W1600)
                .await
            {
                Ok(_) => {}
                Err(e) => warn!("Failed to upload W400 photo: {e}"),
            },
            Err(e) => warn!("Failed to scale to W400: {e}"),
        }
    });

    Ok(Payload(CreatePhotoResponse { photo_id: photo_id_for_response }))
}

fn strip_exif_metadata(image_bytes: Vec<u8>) -> Result<Vec<u8>, Error> {
    // Strip exif metadata
    let mut dyn_image = DynImage::from_bytes(Bytes::from(image_bytes))
        .tap_err(|e| warn!("Failed to create DynImage (stripping EXIF metadata): {e}"))
        .map_err(|_| Error::ImageEncoding)?
        .unwrap();

    let mut image_bytes = Vec::new();
    dyn_image.set_exif(None);
    dyn_image
        .encoder()
        .write_to(&mut image_bytes)
        .tap_err(|e| warn!("Failed to reencode image (stripping EXIF metadata): {e}"))
        .map_err(|_| Error::ImageEncoding)?;

    Ok(image_bytes)
}

fn try_parse_exif_timestamp(image_bytes: Vec<u8>) -> Result<i64, Error> {
    // Parse EXIF metadata
    let exif = exif::Reader::new().read_raw(image_bytes)
        .map_err(|_| Error::ImageEncoding)?;

    // Try to parse the timestamp the photo was created at from EXIF metadata.
    // If it is not available, use the current server time.
    let timestamp = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)
        .map(|field| match &field.value {
            exif::Value::Ascii(ascii) => {
                // Field value is documented as 'Vector of slices of 8-bit bytes containing 7-bit ASCII characters'
                // Merge all inner vectors into one
                let ascii = ascii
                    .clone()
                    .into_iter()
                    .reduce(|mut acc, mut item| {
                        acc.append(&mut item);
                        acc
                    })
                    .unwrap_or(Vec::new());

                // Conver to a string
                String::from_utf8(ascii)
                    // Try to parse the datetime to an epoch timestamp
                    .map(|datetime| chrono::DateTime::<chrono::offset::Utc>::from_str(&datetime)
                        .map(|datetime| datetime.timestamp())
                        .map_err(|_| Error::ImageEncoding))
                    .map_err(|_| Error::ImageEncoding)
            },
            _ => Err(Error::ImageEncoding)
        })
        .unwrap_or(Ok(Ok(OffsetDateTime::now_utc().unix_timestamp())))??;

    Ok(timestamp)
}

fn convert_quality(img: &DynamicImage, target_width: u32) -> color_eyre::Result<Vec<u8>> {
    let (width, height) = img.dimensions();

    debug!("Converting {width}x{height} to W{target_width}");

    let target_height = (height as f32 / (width as f32 / target_width as f32)).round() as u32;
    let scaled = if target_width > width {
        img.resize(target_width, target_height, FilterType::Nearest)
    } else {
        img.thumbnail(target_width, target_height)
    };

    let encoder = Encoder::from_image(&scaled)
        .tap_err(|e| warn!("Failed to create image encoder: {e}"))
        .map_err(|_| Error::ImageEncoding)?;
    let encoded_webp = encoder.encode(100.0);

    Ok(encoded_webp.to_vec())
}
