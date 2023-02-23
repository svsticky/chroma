use actix_web::web::ServiceConfig;

pub trait Routable {
    fn configure(config: &mut ServiceConfig);
}