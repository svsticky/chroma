use actix_web::web;
use actix_web::web::ServiceConfig;
use routable::Routable;

pub mod appdata;
mod authorization;
mod empty;
mod error;
mod redirect;
pub mod routable;
mod v1;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/api").configure(v1::Router::configure));
    }
}
