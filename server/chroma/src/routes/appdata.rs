use crate::config::Config;
use actix_web::web;
use dal::database::Database;
use dal::s3::S3;

pub type WebData = web::Data<AppData>;

#[derive(Debug, Clone)]
pub struct AppData {
    pub db: Database,
    pub s3: S3,
    pub config: Config,
}
