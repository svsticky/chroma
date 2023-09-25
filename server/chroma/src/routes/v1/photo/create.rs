use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, ImagePipelineError, WebResult};
use actix_multiresponse::Payload;
use dal::database::{Album, Photo};
use dal::storage_engine::{PhotoQuality, StorageEngine};
use exif::{In, Tag};
use image::imageops::FilterType;
use image::io::Reader;
use image::{DynamicImage, GenericImageView};
use img_parts::{Bytes, DynImage, ImageEXIF};
use proto::{CreatePhotoRequest, CreatePhotoResponse};
use std::io::Cursor;
use std::str::FromStr;
use tap::TapFallible;
use time::OffsetDateTime;
use tracing::{debug, trace, warn};
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

    // TODO Update actix-multiresponse to support moving out the payload, avoids another clone
    let photo_id = image_pipeline(&data, payload.photo_data.clone(), &album).await?;

    Ok(Payload(CreatePhotoResponse { photo_id }))
}

/// Process the image and return the resulting photo ID
///
/// # Errors
///
/// If any step in the pipeline fails
async fn image_pipeline(data: &WebData, image: Vec<u8>, album: &Album<'_>) -> WebResult<String> {
    // This pipeline modifies the image. The idea is that each 'step' outputs
    // a variable 'image', which the next step can then use.

    let image = Reader::new(Cursor::new(image))
        .with_guessed_format()
        .unwrap(); // Cannot fail when using a Cursor

    // Decode image to DynamicImage
    let image = image.decode()?;

    // Re-Encode to WebP
    let image = Encoder::from_image(&image)
        .map_err(|e| ImagePipelineError::WebpEncoding(e.to_string()))?
        .encode(100.0);

    // Parse EXIF timestamp, if available
    let timestamp = try_parse_exif_timestamp(image.to_vec())
        .tap_err(|e| {
            warn!("Failed to extract timestamp from EXIF data: {e}. Using current time instead")
        })
        .unwrap_or(OffsetDateTime::now_utc().unix_timestamp());

    // Create the photo metadata
    let photo_metadata = Photo::create(&data.db, &album, timestamp).await?;

    // Strip EXIF metadata
    let image = strip_exif_metadata(image.to_vec())?;

    // Upload original quality image
    tokio::spawn(save_to_engine(
        data.storage.clone(),
        image.clone(),
        photo_metadata.id.clone(),
        PhotoQuality::Original,
    ));

    // Decode WebP image to DynamicImage again
    let image = webp::Decoder::new(&image)
        .decode()
        .unwrap() // It is guaranteed to be a WebP image
        .to_image();

    resize_and_save(
        image.clone(),
        PhotoQuality::W400,
        data.storage.clone(),
        photo_metadata.id.clone(),
    );

    resize_and_save(
        image,
        PhotoQuality::W1600,
        data.storage.clone(),
        photo_metadata.id.clone(),
    );

    Ok(photo_metadata.id)
}

/// Resize an image and save the resulting image.
/// Spawns a new Tokio task.
fn resize_and_save(
    image: DynamicImage,
    quality: PhotoQuality,
    engine: StorageEngine,
    photo_id: String,
) {
    let target_width = match quality.width() {
        Some(w) => w,
        None => {
            warn!("Programmer: You wanted to resize an image to its 'original' quality, that doesn't work! Ignoring.");
            return;
        }
    };

    tokio::spawn(async move {
        trace!("Converting image to W{target_width}");
        match convert_quality(&image, target_width) {
            Ok(data) => save_to_engine(engine, data, &photo_id, quality).await,
            Err(e) => warn!("Failed to scale to W{target_width}: {e}"),
        }
    });
}

/// Save the provided data.
/// Errors are logged, rather than bubbled up.
async fn save_to_engine(
    engine: StorageEngine,
    bytes: Vec<u8>,
    id: impl AsRef<str>,
    quality: PhotoQuality,
) {
    trace!("Saving image '{}' in quality '{quality:?}'", id.as_ref());
    match engine.create_photo(id.as_ref(), bytes, quality).await {
        Ok(_) => {}
        Err(e) => warn!("Failed to upload photo: {e}"),
    }
}

/// Strip the provided image of its EXIF metadata.
/// The image provided should be a WebP image.
///
/// # Errors
///
/// If decoding or encoding the image fails
fn strip_exif_metadata(image_bytes: Vec<u8>) -> Result<Vec<u8>, ImagePipelineError> {
    trace!("Stripping EXIF metadata");

    // Strip exif metadata
    let mut dyn_image = DynImage::from_bytes(Bytes::from(image_bytes))
        .tap_err(|e| warn!("Failed to create DynImage (stripping EXIF metadata): {e}"))?
        .unwrap();

    let mut image_bytes = Vec::new();
    dyn_image.set_exif(None);
    dyn_image.encoder().write_to(&mut image_bytes)?;
    Ok(image_bytes)
}

/// Try to parse the EXIF timestamp from the image.
///
/// # Errors
/// -
fn try_parse_exif_timestamp(image_bytes: Vec<u8>) -> Result<i64, ImagePipelineError> {
    // Parse EXIF metadata
    let exif = exif::Reader::new().read_raw(image_bytes)?;

    // Try to parse the timestamp the photo was created at from EXIF metadata.
    // If it is not available, use the current server time.
    let timestamp = exif
        .get_field(Tag::DateTimeOriginal, In::PRIMARY)
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
                    .ok_or(ImagePipelineError::MissingExifField("DateTimeOriginal"))?;

                // Conver to a string
                let datetime_string = String::from_utf8(ascii)?;
                // Try to parse the datetime
                let datetime = chrono::DateTime::<chrono::offset::Utc>::from_str(&datetime_string)?;

                Ok(datetime.timestamp())
            }
            _ => Err(ImagePipelineError::InvalidExifFieldType("DateTimeOriginal")),
        })
        .ok_or(ImagePipelineError::MissingExifField("DateTimeOriginal"))??;

    Ok(timestamp)
}

/// Convert an image to the provided target width.
/// The height of the image will be scaled such that the aspect ratio remains the same.
///
/// # Errors
///
/// If image encoding fails
fn convert_quality(img: &DynamicImage, target_width: u32) -> Result<Vec<u8>, ImagePipelineError> {
    let (width, height) = img.dimensions();

    debug!("Converting {width}x{height} to W{target_width}");

    let target_height = (height as f32 / (width as f32 / target_width as f32)).round() as u32;
    let scaled = if target_width > width {
        img.resize(target_width, target_height, FilterType::Nearest)
    } else {
        img.thumbnail(target_width, target_height)
    };

    let encoder = Encoder::from_image(&scaled)
        .map_err(|e| ImagePipelineError::WebpEncoding(e.to_string()))?;
    let encoded_webp = encoder.encode(100.0);

    Ok(encoded_webp.to_vec())
}
