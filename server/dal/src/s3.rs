use aws_credential_types::Credentials;
use aws_sdk_s3::{AppName, Client, Config, Region};
use thiserror::Error;

pub mod aws_errors {
    pub use aws_sdk_s3::error::*;
}


#[derive(Debug, Clone)]
pub struct S3(Client);

#[derive(Debug, Error)]
pub enum S3InitError {
    #[error("Invalid App name provided")]
    AppName(#[from] aws_types::app_name::InvalidAppName),
}

pub async fn init_s3(app_name: String, endpoint_url: &str, region: String, access_key_id: &str, secret_access_key: &str) -> Result<S3, S3InitError> {
    let client = aws_sdk_s3::Client::from_conf(Config::builder()
        .app_name(AppName::new(app_name)?)
        .endpoint_url(endpoint_url)
        .region(Some(Region::new(region)))
        .credentials_provider(Credentials::from_keys(access_key_id, secret_access_key, None))
        .build()
    );

    Ok(S3(client))
}