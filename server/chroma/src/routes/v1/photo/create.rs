use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};
use actix_multiresponse::Payload;
use dal::database::{Album, Photo};
use image::io::Reader;
use image::{ImageFormat, ImageOutputFormat};
use proto::CreatePhotoRequest;
use std::io::Cursor;

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
    if let Err(e) = data.s3.create_photo(&photo.id, png_image).await {
        photo.delete().await?;
        return Err(e.into());
    }

    Ok(Empty)
}
