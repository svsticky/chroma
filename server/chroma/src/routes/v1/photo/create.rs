use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, ImagePipelineError, WebResult};
use actix_multiresponse::Payload;
use dal::database::{Album, Database, Photo, PhotoQuality};
use dal::storage_engine::Storage;
use exif::{In, Tag};
use image::imageops::FilterType;
use image::io::Reader;
use image::{DynamicImage, GenericImageView};
use img_parts::{Bytes, DynImage, ImageEXIF};
use proto::photo_respone::Response;
use proto::{CreatePhotoRequest, CreatePhotoResponse};
use std::io::Cursor;
use tap::TapFallible;
use time::OffsetDateTime;
use tokio::time::Instant;
use tracing::{debug, instrument, trace, warn};
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
    if !auth.is_admin
        && !auth
            .has_scope(&data.db, "nl.svsticky.chroma.photo.create")
            .await?
    {
        return Err(Error::Forbidden);
    }

    let album = Album::get_by_id(&data.db, &payload.album_id)
        .await?
        .ok_or(Error::NotFound)?;

    // Album must be un-published for non-admins to modify them.
    if !album.is_draft && !auth.is_admin {
        return Err(Error::Forbidden);
    }

    // TODO Update actix-multiresponse to support moving out the payload, avoids another clone
    let photo_id = image_pipeline(&data, payload.photo_data.clone(), &album, &data.db).await?;

    Ok(Payload(CreatePhotoResponse { photo_id }))
}

/// Process the image and return the resulting photo ID
///
/// # Errors
///
/// If any step in the pipeline fails
#[instrument(skip(data, image))]
async fn image_pipeline(
    data: &WebData,
    image: Vec<u8>,
    album: &Album,
    db: &Database,
) -> WebResult<String> {
    // Make sure we don't run into AWS ratelimits here
    if let Err(e) = data.ratelimits.photo_create.check() {
        return Err(Error::Ratelimit {
            retry_after: e.wait_time_from(e.earliest_possible()).as_secs()
        });
    }

    // This pipeline modifies the image. The idea is that each 'step' outputs
    // a variable 'image', which the next step can then use.

    // We want to keep track of the duration of various steps.
    let mut timer = Instant::now();

    // EXIF metadata seems to get stripped in the decoding process. Extract the timestamp before that happens
    let timestamp = try_parse_exif_timestamp(image.clone())
        .tap_err(|e| {
            warn!("Failed to extract timestamp from EXIF data: {e}. Using current time instead")
        })
        .unwrap_or(OffsetDateTime::now_utc().unix_timestamp());

    trace!(
        "Parsing EXIF timestamp took {} ms",
        timer.elapsed().as_millis()
    );
    timer = Instant::now();

    // Decoding the image also removes all EXIF metadata, we do not need to strip it manually as well.
    trace!("Decoding received image");
    let image = Reader::new(Cursor::new(image))
        .with_guessed_format()
        .unwrap() // Cannot fail when using a Cursor
        .decode()?;

    trace!(
        "Decoding received image to DynamicImage took {} ms",
        timer.elapsed().as_millis()
    );

    // Create the photo metadata
    let photo_metadata = Photo::create(&data.db, album, timestamp).await?;

    // Upload original quality image
    trace!("Uploading original image on another Task");
    let original_image = image.clone();
    let photo_id = photo_metadata.id.clone();
    let engine = data.storage.clone();
    tokio::spawn(async move {
        // Encode to WebP
        let encoder = match Encoder::from_image(&original_image) {
            Ok(encoder) => encoder,
            Err(e) => {
                warn!("{e}");
                return;
            }
        };

        let image = encoder.encode(100.0).to_vec();

        // Upload
        trace!(
            "Saving image '{photo_id}' in quality '{:?}'",
            PhotoQuality::Original
        );
        match engine
            .create_photo(&photo_id, &PhotoQuality::Original, image)
            .await
        {
            Ok(_) => {}
            Err(e) => {
                warn!("Failed to upload photo: {e}");
            }
        }
    });

    trace!("Resizing to W400 on another Task");
    resize_and_save(
        image.clone(),
        PhotoQuality::W400,
        data.storage.clone(),
        data.db.clone(),
        photo_metadata.id.clone(),
    );

    trace!("Resizing to W1600 on another Task");
    resize_and_save(
        image,
        PhotoQuality::W1600,
        data.storage.clone(),
        data.db.clone(),
        photo_metadata.id.clone(),
    );

    Ok(photo_metadata.id)
}

/// Resize an image and save the resulting image.
/// Spawns a new Tokio task.
#[instrument(skip(image, engine, db))]
fn resize_and_save(
    image: DynamicImage,
    quality: PhotoQuality,
    engine: Storage,
    db: Database,
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
        let converted_image_data = match convert_quality(&image, target_width) {
            Ok(data) => data,
            Err(e) => {
                warn!("Failed to scale to W{target_width}: {e}");
                return;
            }
        };

        trace!("Saving image '{photo_id}' in quality '{quality:?}'");
        match engine
            .create_photo(&photo_id, &quality, converted_image_data)
            .await
        {
            Ok(_) => {}
            Err(e) => {
                warn!("Failed to upload photo: {e}");
                return;
            }
        }

        let photo = match Photo::get_by_id(&db, &photo_id).await {
            Ok(Some(p)) => p,
            Ok(None) => {
                warn!("Unable to get photo metadata: It doesn't exist");
                return;
            }
            Err(e) => {
                warn!("Failed to get photo metadata: {e}");
                return;
            }
        };

        // Verify, this also puts it in the cache, nice speedup for later
        trace!("Checking if uploaded image actually works on AWS");

        let url = match photo.photo_to_proto_url(&engine, &quality).await {
            Ok(p) => match p.data.unwrap().response.unwrap() {
                Response::Url(v) => v,
                _ => panic!("Invalid response type for the 'URL' method"),
            },
            Err(e) => {
                warn!("Photo {} with quality {} was not created successfully, or another error occurred: {e}", photo_id, quality);
                return;
            }
        };

        // Fetch the photo
        let ok = reqwest::Client::new().get(url).send().await.is_ok();

        if !ok {
            warn!(
                "Photo {} with quality {} was not created successfully (AWS returned an error)",
                photo_id, quality
            );
            return;
        }
    });
}

/// Try to parse the EXIF timestamp from the image.
///
/// # Errors
/// -
fn try_parse_exif_timestamp(image_bytes: Vec<u8>) -> Result<i64, ImagePipelineError> {
    // Create a DynImage to extract EXIF data
    let dyn_image = DynImage::from_bytes(Bytes::from(image_bytes))
        .tap_err(|e| warn!("Failed to create DynImage (stripping EXIF metadata): {e}"))?
        .ok_or(ImagePipelineError::MissingExifField("All"))?;

    let exif = exif::Reader::new()
        .read_raw(
            dyn_image
                .exif()
                .ok_or(ImagePipelineError::MissingExifField("All"))?
                .to_vec(),
        )
        .tap_err(|_| warn!("A"))?;

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

                // Convert the ASCII bytes to a UTF-8 string
                let datetime_string = String::from_utf8(ascii)?;

                // The datetime stored isn't in the format we can use for parsing.
                // Convert it to RFC 3339 format.
                let components = datetime_string.trim().split(' ').collect::<Vec<_>>();
                let date = components
                    .first()
                    .ok_or(ImagePipelineError::InvalidExifFieldType("DateTimeOriginal"))?;
                let time = components
                    .get(1)
                    .ok_or(ImagePipelineError::InvalidExifFieldType("DateTimeOriginal"))?;

                // Replace ':' with '-' in date
                let date = date.replace(':', "-");

                // Join them using the RFC 3339 seperator, 'T' and add a 'Z' at the end.
                let datetime_string = format!("{date}T{time}Z");

                // Try to parse the datetime
                let datetime = chrono::DateTime::parse_from_rfc3339(datetime_string.trim())?;
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
