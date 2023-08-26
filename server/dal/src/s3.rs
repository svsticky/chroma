use aws_credential_types::Credentials;
use aws_sdk_s3::error::{DeleteObjectError, GetObjectError, GetObjectErrorKind, PutObjectError};
use aws_sdk_s3::types::{ByteStream, SdkError};
use aws_sdk_s3::{Client, Config, Region};
use std::ops::Deref;
use async_recursion::async_recursion;
use strum_macros::Display;
use thiserror::Error;

pub mod aws_errors {
    pub use aws_sdk_s3::error::*;
}

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
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum PhotoQuality {
    Original,
    W400,
    W1600,
}

pub struct S3Config {
    pub bucket_name: String,
    pub endpoint_url: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub use_path_style: bool,
}

fn fmt_id_with_quality(id: &str, quality: PhotoQuality) -> String {
    format!("{id}_{quality}")
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

    #[async_recursion]
    pub async fn get_photo_by_id<S: AsRef<str> + Send + Sync>(&self, photo_id: S, quality_preference: PhotoQuality) -> Result<Vec<u8>, S3Error> {
        let photo = self
            .get_object()
            .bucket(&self.bucket_name)
            .key(&fmt_id_with_quality(photo_id.as_ref(), quality_preference.clone()))
            .send()
            .await;

        match photo {
            Ok(photo) => {
                let bytes = photo.body.collect().await?;
                let bytes = bytes.to_vec();
                Ok(bytes)
            },
            Err(e) => {
                match &e {
                    SdkError::ServiceError(svc_e) => match svc_e.err().kind {
                        GetObjectErrorKind::NoSuchKey(_) => {
                            if quality_preference == PhotoQuality::Original {
                                Err(e.into())
                            } else {
                                self.get_photo_by_id(photo_id.as_ref(), PhotoQuality::Original).await
                            }
                        },
                        _ => Err(e.into())
                    },
                    _ => Err(e.into())
                }
            }
        }
    }

    pub async fn create_photo<S: AsRef<str>>(
        &self,
        photo_id: S,
        bytes: Vec<u8>,
        quality: PhotoQuality,
    ) -> Result<(), S3Error> {
        let byte_stream = ByteStream::from(bytes);

        self.put_object()
            .bucket(&self.bucket_name)
            .key(&fmt_id_with_quality(photo_id.as_ref(), quality))
            .body(byte_stream)
            .send()
            .await?;

        Ok(())
    }

    pub async fn delete_photo<S: AsRef<str>>(&self, photo_id: S) -> Result<(), S3Error> {
        self.delete_photo_with_quality(photo_id.as_ref(), PhotoQuality::Original).await?;
        self.delete_photo_with_quality(photo_id.as_ref(), PhotoQuality::W1600).await?;
        self.delete_photo_with_quality(photo_id.as_ref(), PhotoQuality::W400).await?;
        Ok(())
    }

    async fn delete_photo_with_quality(&self, photo_id: &str, quality: PhotoQuality) -> Result<(), S3Error> {
        self.delete_object()
            .bucket(&self.bucket_name)
            .key(&fmt_id_with_quality(photo_id, quality))
            .send()
            .await?;
        Ok(())
    }
}
