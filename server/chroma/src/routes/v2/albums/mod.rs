use actix_web::web;
use actix_web::web::ServiceConfig;

use crate::routes::routable::Routable;

mod create;
mod delete;
mod get;
mod list;
mod update;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/albums")
                .route("", web::get().to(list::list))
                .route("", web::post().to(create::create))
                .route("/{id}", web::get().to(get::get))
                .route("/{id}", web::patch().to(update::update))
                .route("/{id}", web::delete().to(delete::delete)),
        );
    }
}
