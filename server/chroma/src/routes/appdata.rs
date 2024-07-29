use std::num::NonZeroU32;
use std::sync::Arc;

use actix_web::web;
use cabbage::KoalaApi;
use governor::{Quota, RateLimiter};
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use moka::future::Cache;

use dal::database::{Album, Database};
use dal::storage_engine::Storage;

use crate::config::Config;
use crate::routes::authorization::Authorization;

pub type WebData = web::Data<AppData>;
pub type SessionIdCache = Cache<String, Authorization>;
pub type AlbumIdCache = Cache<String, Album>;

#[derive(Debug, Clone)]
pub struct AppData {
    pub db: Database,
    pub storage: Storage,
    pub config: Config,
    pub koala: KoalaApi,
    pub ratelimits: Ratelimits,
}

#[derive(Debug, Clone)]
pub struct Ratelimits {
    pub photo_create: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl Ratelimits {
    pub fn new() -> Self {
        Self {
            photo_create: Arc::new(RateLimiter::direct(Quota::per_second(
                NonZeroU32::new(1).unwrap(),
            ))),
        }
    }
}
