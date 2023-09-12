use crate::routes::routable::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod available_scopes;
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
                .route("/list", web::get().to(list::list))
                .route(
                    "/available-scopes",
                    web::get().to(available_scopes::available_scopes),
                ),
        );
    }
}
