use actix_web::web;
use actix_web::web::ServiceConfig;
use routable::Routable;

mod error;
pub mod routable;
mod v1;
pub mod appdata;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/api")
            .configure(v1::Router::configure)
        );
    }
}