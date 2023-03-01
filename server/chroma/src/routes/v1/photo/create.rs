use actix_multiresponse::Payload;
use tracing::{debug, info};
use dal::database::{Album, Photo};
use proto::CreatePhotoRequest;
use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};

/// Create a new photo in an existing album.
///
/// # Errors
///
/// - If the album does not exist
/// - If something went wrong
pub async fn create(auth: Authorization, data: WebData, payload: Payload<CreatePhotoRequest>) -> WebResult<Empty> {
    if !auth.is_admin {
        return Err(Error::Forbidden);
    }

    let album = Album::get_by_id(&data.db, &payload.album_id)
        .await?
        .ok_or(Error::NotFound)?;

    // Create the photo metadata in the DB
    let photo = Photo::create(&data.db, &album).await?;

    // Upload the photo to S3
    // If this fails, remove the metadata again
    info!("{:?}", data.s3);
    if let Err(e) = data.s3.create_photo(&photo.id, payload.photo_data.clone()).await {
        photo.delete().await?;
        return Err(e.into());
    }

    Ok(Empty)
}