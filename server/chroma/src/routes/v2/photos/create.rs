use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, ImagePipelineError, WebResult};
use actix_multiresponse::Payload;
use chrono::NaiveDate;
use dal::database::{Photo, PhotoUrl};
use dal::database::{PhotoAlbum, PhotoExif};
use exif::{Exif, In, Tag, Value};
use imagesize::ImageType;
use proto::CreatePhotoRequest;
use std::io::Cursor;

const EXIF_FIELDS: [Tag; 40] = [
    // Camera fields
    Tag::Make,
    Tag::Model,
    Tag::XResolution,
    Tag::YResolution,
    Tag::ResolutionUnit,
    Tag::Software,
    Tag::ExifVersion,
    Tag::FocalLengthIn35mmFilm,
    // Image Data
    Tag::ImageDescription,
    Tag::DateTime,
    Tag::Copyright,
    Tag::Compression,
    Tag::ISOSpeed,
    Tag::DateTimeOriginal,
    Tag::DateTimeDigitized,
    Tag::ExposureTime,
    Tag::FNumber,
    Tag::ExposureProgram,
    Tag::ShutterSpeedValue,
    Tag::ApertureValue,
    Tag::MaxApertureValue,
    Tag::MeteringMode,
    Tag::LightSource,
    Tag::Flash,
    Tag::FocalLength,
    Tag::SensingMethod,
    Tag::SceneType,
    Tag::ExposureMode,
    Tag::WhiteBalance,
    Tag::DigitalZoomRatio,
    Tag::SceneCaptureType,
    Tag::Sharpness,
    // GPS Data
    Tag::GPSLatitudeRef,
    Tag::GPSLatitude,
    Tag::GPSLongitudeRef,
    Tag::GPSLongitude,
    Tag::GPSAltitudeRef,
    Tag::GPSAltitude,
    Tag::GPSTimeStamp,
    Tag::GPSDateStamp,
];

/// Create a new photo in an existing albums.
///
/// # Errors
///
/// - If the album does not exist
/// - If something went wrong
pub async fn create(
    auth: Authorization,
    data: WebData,
    payload: Payload<CreatePhotoRequest>,
) -> WebResult<Payload<proto::Photo>> {
    if !auth.is_admin
        && !auth
            .has_scope(&data.db, "nl.svsticky.chroma.photos.create")
            .await?
    {
        return Err(Error::Forbidden);
    }

    // Calculate the hash
    let hash = md5::compute(&payload.data);

    // Read all exif fields
    let exif = parse_exif(&payload.data).ok();

    // Create the photos metadata
    let mut photo = Photo::create(
        &data.db,
        format!("{:x}", hash),
        exif.as_ref().and_then(get_exif_capture_date),
    )
    .await?;

    if let Some(exif) = &exif {
        for (field, value) in get_exif_fields(exif) {
            photo
                .exif_fields
                .push(PhotoExif::create(&data.db, &photo.id, &field, &value).await?);
        }
    }

    if let Some(album_id) = &payload.album_id {
        photo
            .linked_albums
            .push(PhotoAlbum::create(&data.db, &photo.id, album_id).await?);
    }

    // Get the image size
    let mut image_size = imagesize::blob_size(&payload.data).map_err(Error::ImageSize)?;

    // Check if the image has been rotated using exif tags
    if exif
        .as_ref()
        .and_then(get_exif_rotation)
        .map(|rotation| rotation >= 5)
        .unwrap_or(false)
    {
        std::mem::swap(&mut image_size.width, &mut image_size.height);
    }

    let image_type = match imagesize::image_type(&payload.data).map_err(Error::ImageSize)? {
        ImageType::Jpeg => "image/jpeg",
        ImageType::Png => "image/png",
        ImageType::Tiff => "image/tiff",
        ImageType::Webp => "image/webp",
        _ => "application/octet-stream",
    };

    match data
        .storage
        .create_photo(&photo.id, "Original", payload.data.clone(), image_type)
        .await
    {
        Ok(_) => photo.media_urls.push(
            PhotoUrl::create(
                &data.db,
                &photo.id,
                &data.storage.get_photo_url_by_id(&photo.id, "Original")?,
                "Original",
                image_size.width as i32,
                image_size.height as i32,
            )
            .await?,
        ),
        Err(e) => {
            Photo::delete(&data.db, &photo.id).await?;
            Err(e)?
        }
    };

    Ok(Payload(photo.into()))
}

// Try to parse the EXIF timestamp from the image.
//
// # Errors
//
fn parse_exif(image: &[u8]) -> Result<Exif, ImagePipelineError> {
    let mut cursor = Cursor::new(image);
    Ok(exif::Reader::new().read_from_container(&mut cursor)?)
}

fn get_exif_capture_date(exif: &Exif) -> Option<i64> {
    exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)
        .or_else(|| exif.get_field(Tag::DateTimeDigitized, In::PRIMARY))
        .or_else(|| exif.get_field(Tag::DateTime, In::PRIMARY))
        .and_then(|datetime_field| match &datetime_field.value {
            Value::Ascii(ascii_values) => ascii_values.first(),
            _ => None,
        })
        .and_then(|datetime_string| exif::DateTime::from_ascii(datetime_string).ok())
        .and_then(|datetime| {
            Some(
                NaiveDate::from_ymd_opt(
                    datetime.year as i32,
                    datetime.month as u32,
                    datetime.day as u32,
                )?
                .and_hms_opt(
                    datetime.hour as u32,
                    datetime.minute as u32,
                    datetime.second as u32,
                )?
                .and_utc()
                .timestamp(),
            )
        })
}

fn get_exif_rotation(exif: &Exif) -> Option<u32> {
    exif.get_field(Tag::Orientation, In::PRIMARY)
        .and_then(|orientation| match orientation.value.get_uint(0) {
            Some(v @ 1..=8) => Some(v),
            _ => None,
        })
}

fn get_exif_fields(exif: &Exif) -> Vec<(String, String)> {
    exif.fields()
        .filter(|field| field.ifd_num == In::PRIMARY && EXIF_FIELDS.contains(&field.tag))
        .map(|field| (field.tag.to_string(), format!("{}", field.display_value())))
        .collect()
}
