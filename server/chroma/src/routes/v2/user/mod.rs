use actix_web::web;
use actix_web::web::ServiceConfig;

use crate::routes::routable::Routable;

mod get;
mod list;
mod update;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/user")
                .route("", web::get().to(get::get))
                .route("", web::patch().to(update::update))
                .route("/list", web::get().to(list::list)),
        );
    }
}
