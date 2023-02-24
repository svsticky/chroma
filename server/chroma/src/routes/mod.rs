use actix_web::web;
use actix_web::web::ServiceConfig;
use routable::Routable;

pub mod appdata;
pub mod routable;
mod error;
mod v1;
mod empty;
mod authorization;
mod redirect;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/api")
            .configure(v1::Router::configure)
        );
    }
}