use actix_web::web;
use actix_web::web::ServiceConfig;

use crate::routes::routable::Routable;

mod batch_delete;
mod batch_get;
mod create;
mod delete;
mod get;
mod jobs;
mod search;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config
            .route("photos:batchGet", web::post().to(batch_get::batch_get))
            .route(
                "photos:batchDelete",
                web::post().to(batch_delete::batch_delete),
            )
            .service(
                web::scope("/photos")
                    .route("", web::post().to(create::create))
                    .route("/search", web::post().to(search::search))
                    .route("/{id}", web::get().to(get::get))
                    .route("/{id}", web::delete().to(delete::delete))
                    .service(web::scope("/{id}").configure(jobs::Router::configure)),
            );
    }
}
