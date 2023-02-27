use std::time::Duration;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_governor::governor::middleware::{NoOpMiddleware, StateInformationMiddleware};
use actix_web::middleware::Compat;
use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::routes::routable::Routable;

mod album;
mod photo;
mod login;
mod access;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/v1")
            .configure(album::Router::configure)
            .configure(photo::Router::configure)
            .route("/login", web::get().to(login::login))
            // This route requires strict ratelimits
            // We allow one request every 2 seconds per IP.
            .service(web::scope("/access")
                .wrap(Governor::new(&access_ratelimit(Duration::from_secs(2))))
                .route("", web::get().to(access::access))
            )
        );
    }
}

/// Get a ratelimiter config for the `/access` endpoint
///
/// `requests_per_n` defines the interval between each request.
/// E.g. if this is a duration of 5 seocnds, there may be one request every 5 seconds.
/// Once this is exceeded, HTTP 429 will be returned
fn access_ratelimit(request_per_n: Duration) -> GovernorConfig<PeerIpKeyExtractor, StateInformationMiddleware> {
    GovernorConfigBuilder::default()
        .period(request_per_n)
        .burst_size(3)
        .use_headers()
        .finish()
        .unwrap()
}