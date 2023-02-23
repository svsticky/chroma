use actix_web::web::ServiceConfig;
use crate::routes::routable::Routable;

mod albums;
mod photo;

pub struct Router;

impl Routable for Router {
    fn configure(_config: &mut ServiceConfig) {
        todo!()
    }
}