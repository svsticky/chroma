use aws_credential_types::Credentials;
use aws_sdk_s3::error::{DeleteObjectError, GetObjectError, PutObjectError};
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Config, Region};
use std::ops::Deref;
use std::time::Duration;
use thiserror::Error;

pub mod aws_errors {
    pub use aws_sdk_s3::error::*;
}

pub use aws_sdk_s3::types::SdkError;

#[derive(Debug, Error)]
pub enum S3InitError {
    #[error("Invalid App name provided")]
    AppName(#[from] aws_types::app_name::InvalidAppName),
}

#[derive(Debug, Clone)]
pub struct S3 {
    client: Client,
    bucket_name: String,
}

impl Deref for S3 {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[derive(Debug, Error)]
pub enum S3Error {
    #[error("Failed to retrieve Object: {0}")]
    GetObject(#[from] SdkError<GetObjectError>),
    #[error("Failed to upload Object: {0}")]
    PutObject(#[from] SdkError<PutObjectError>),
    #[error("Failed to delete Object: {0}")]
    DeleteObject(#[from] SdkError<DeleteObjectError>),
    #[error("Failed to convert ByteStream: {0}")]
    ByteStream(#[from] aws_smithy_http::byte_stream::error::Error),
    #[error("Failed to create presigning config: {0}")]
    Presigning(#[from] aws_sdk_s3::presigning::config::Error),
}

pub struct S3Config {
    pub bucket_name: String,
    pub endpoint_url: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub use_path_style: bool,
}

impl S3 {
    pub async fn new(config: S3Config) -> Result<S3, S3InitError> {
        let client = Client::from_conf(
            Config::builder()
                .force_path_style(config.use_path_style)
                .endpoint_url(config.endpoint_url)
                .region(Some(Region::new(config.region)))
                .credentials_provider(Credentials::from_keys(
                    config.access_key_id,
                    config.secret_access_key,
                    None,
                ))
                .build(),
        );

        Ok(S3 {
            client,
            bucket_name: config.bucket_name,
        })
    }

    pub async fn get_signed_url_by_id<S: AsRef<str>>(&self, id: S) -> Result<String, S3Error> {
        let response = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(id.as_ref())
            .presigned(PresigningConfig::expires_in(Duration::from_secs(6000))?)
            .await?;

        let url = response.uri();
        Ok(url.to_string())
    }

    pub async fn get_photo_by_id<S: AsRef<str>>(&self, photo_id: S) -> Result<Vec<u8>, S3Error> {
        let photo = self
            .get_object()
            .bucket(&self.bucket_name)
            .key(photo_id.as_ref())
            .send()
            .await?;

        let bytes = photo.body.collect().await?;
        let bytes = bytes.to_vec();

        Ok(bytes)
    }

    pub async fn create_photo<S: AsRef<str>>(
        &self,
        photo_id: S,
        bytes: Vec<u8>,
    ) -> Result<(), S3Error> {
        let byte_stream = ByteStream::from(bytes);

        self.put_object()
            .bucket(&self.bucket_name)
            .key(photo_id.as_ref())
            .body(byte_stream)
            .send()
            .await?;

        Ok(())
    }

    pub async fn delete_photo<S: AsRef<str>>(&self, photo_id: S) -> Result<(), S3Error> {
        self.delete_object()
            .bucket(&self.bucket_name)
            .key(photo_id.as_ref())
            .send()
            .await?;
        Ok(())
    }
}
