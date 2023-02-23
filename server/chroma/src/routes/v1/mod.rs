use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::routes::routable::Routable;

mod album;
mod photo;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/v1")
            .configure(album::Router::configure)
        );
    }
}