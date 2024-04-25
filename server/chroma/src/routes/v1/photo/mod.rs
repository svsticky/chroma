use crate::routes::routable::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod create;
mod delete;
mod get;
mod list;
mod report_broken;
pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/photo")
                .route("", web::post().to(create::create))
                .route("", web::delete().to(delete::delete))
                .route("", web::get().to(get::get))
                .route("/list", web::get().to(list::list))
                .route(
                    "/report-broken",
                    web::post().to(report_broken::report_broken),
                ),
        );
    }
}
