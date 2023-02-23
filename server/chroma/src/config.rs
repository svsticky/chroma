use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // Database
    pub db_host: String,
    pub db_database: String,
    pub db_username: String,
    pub db_password: String,

    // S3
    pub s3_app_name: String,
    pub s3_region: String,
    pub s3_endpoint_url: String,
    pub s3_access_key_id: String,
    pub s3_secret_access_key: String,
}

impl Config {
    pub fn parse() -> envy::Result<Self> {
        envy::from_env()
    }
}