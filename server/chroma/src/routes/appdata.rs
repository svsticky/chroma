use crate::config::Config;
use crate::routes::authorization::Authorization;
use actix_web::web;
use cabbage::KoalaApi;
use dal::database::Database;
use dal::storage_engine::StorageEngine;
use moka::future::Cache;

pub type WebData = web::Data<AppData>;
pub type SessionIdCache = Cache<String, Authorization>;

#[derive(Debug, Clone)]
pub struct AppData {
    pub db: Database,
    pub storage: StorageEngine,
    pub config: Config,
    pub koala: KoalaApi,
}
