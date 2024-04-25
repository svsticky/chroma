use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};
use crate::routes::v1::photo::create::resize_and_save;
use actix_multiresponse::Payload;
use dal::database::{Database, Photo};
use dal::storage_engine::{PhotoQuality, StorageEngine, StorageEngineError};
use dal::DalError;
use image::io::Reader;
use proto::photo_respone::Response;
use proto::ReportBrokenPhotoRequest;
use reqwest::{Client, StatusCode};
use std::io::Cursor;

pub async fn report_broken(
    _: Authorization,
    data: WebData,
    payload: Payload<ReportBrokenPhotoRequest>,
) -> WebResult<Empty> {
    let photo_quality = proto::PhotoQuality::from_i32(payload.photo_quality)
        .ok_or(Error::BadRequest("Invalid photo quality value".to_string()))?;
    let photo_quality = photo_quality_ptd(photo_quality);

    if photo_quality.eq(&PhotoQuality::Original) {
        return Err(Error::BadRequest(
            "If the original image is broken, it cannot be fixed.".to_string(),
        ));
    }

    let photo = Photo::get_by_id(&data.db, &payload.id)
        .await?
        .ok_or(Error::NotFound)?;
    let proto = match photo
        .clone()
        .photo_to_proto_url(&data.storage, photo_quality.clone())
        .await
    {
        Ok(v) => v,
        Err(e) => {
            return match e {
                DalError::Storage(e) => match e {
                    // URL mode is not supported
                    StorageEngineError::NotSupported => {
                        Err(Error::BadRequest("Cannot happen".to_string()))
                    }
                    _ => Err(e.into()),
                },
                DalError::Db(e) => Err(e.into()),
            };
        }
    };

    let url = match proto
        .data
        .ok_or(Error::Other(StatusCode::INTERNAL_SERVER_ERROR))?
        .response
        .ok_or(Error::Other(StatusCode::INTERNAL_SERVER_ERROR))?
    {
        Response::Url(v) => v,
        Response::Bytes(_) => return Err(Error::Other(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    // One of error or does it always error
    if check_url(&url).await? {
        return Ok(Empty);
    }

    // Recreate the image
    replace_photo(photo, photo_quality, data.storage.clone(), data.db.clone()).await?;

    Ok(Empty)
}

fn photo_quality_ptd(p: proto::PhotoQuality) -> PhotoQuality {
    match p {
        proto::PhotoQuality::Original => PhotoQuality::Original,
        proto::PhotoQuality::W400 => PhotoQuality::W400,
        proto::PhotoQuality::W1600 => PhotoQuality::W1600,
    }
}

async fn check_url(url: &str) -> WebResult<bool> {
    let res = Client::new()
        .get(url)
        .header(
            "User-Agent",
            format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        )
        .send()
        .await;

    match res {
        Ok(t) => Ok(t.bytes().await.is_ok()),
        Err(_) => Ok(false),
    }
}

async fn replace_photo(
    photo: Photo<'_>,
    quality: PhotoQuality,
    engine: StorageEngine,
    db: Database,
) -> WebResult<()> {
    let original_quality = engine
        .get_photo_by_id(&photo.id, PhotoQuality::Original)
        .await?;
    let image = Reader::new(Cursor::new(original_quality))
        .with_guessed_format()
        .unwrap() // Cannot fail when using a Cursor
        .decode()?;

    resize_and_save(image, quality, engine, db, photo.id);

    Ok(())
}
