use actix_web::web;
use actix_web::web::ServiceConfig;
use dal::database::Photo;
use dal::s3::S3;
use crate::routes::error::WebResult;
use crate::routes::routable::Routable;

mod create;
mod delete;
mod get;
mod list;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/photo")
            .route("/", web::post().to(create::create))
            .route("/", web::delete().to(delete::delete))
            .route("/", web::get().to(get::get))
            .route("/list", web::get().to(list::list))
        );
    }
}

/// Convert a [Photo] to a [proto::Photo].
/// Retrieves the photo's content from S3.
///
/// # Errors
///
/// If fetching the photo's contents from S3 failed
pub async fn photo_to_proto(s3: &S3, photo: Photo<'_>) -> WebResult<proto::Photo> {
    let photo_bytes = s3.get_photo_by_id(&photo.id).await?;
    Ok(proto::Photo {
        id: photo.id,
        album_id: photo.album_id,
        created_at: photo.created_at,
        photo_data: photo_bytes
    })
}