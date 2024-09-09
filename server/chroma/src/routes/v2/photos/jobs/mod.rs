use actix_web::web;
use actix_web::web::ServiceConfig;

use crate::routes::routable::Routable;

pub struct Router;
impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/jobs"), // .route("", web::get().to())
                                 // .route("/{id}", web::get().to()),
        );
    }
}
