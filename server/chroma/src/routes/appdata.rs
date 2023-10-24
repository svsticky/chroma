use crate::config::Config;
use actix_web::web;
use cabbage::KoalaApi;
use dal::database::Database;
use dal::storage_engine::StorageEngine;

pub type WebData = web::Data<AppData>;

#[derive(Debug, Clone)]
pub struct AppData {
    pub db: Database,
    pub storage: StorageEngine,
    pub config: Config,
    pub koala: KoalaApi,
}
