use std::ops::Deref;
use std::time::{Duration, Instant};

use aws_credential_types::Credentials;
use aws_sdk_s3::error::HeadBucketError;
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Config};
use aws_smithy_http::result::SdkError;
use aws_types::region::Region;
use strum_macros::Display;
use tracing::log::info;

#[derive(Debug, Clone, PartialEq, Display)]
pub enum PhotoQuality {
    Original,
    W400,
    W1600,
}

impl PhotoQuality {
    pub fn width(&self) -> Option<u32> {
        match self {
            Self::Original => None,
            Self::W400 => Some(400),
            Self::W1600 => Some(1600),
        }
    }
}

pub mod aws_error {
    pub use aws_sdk_s3::error::*;
}

pub mod error {
    use aws_sdk_s3::error::{DeleteObjectError, GetObjectError, HeadBucketError, PutObjectError};
    pub use aws_sdk_s3::types::SdkError;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum InitError {
        #[error("Invalid App name provided")]
        AppName(#[from] aws_types::app_name::InvalidAppName),
        #[error("Failed to retrieve information about Bucket: {0}")]
        HeadBucket(#[from] SdkError<HeadBucketError>),
    }

    #[derive(Debug, Error)]
    pub enum StorageError {
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
}

pub struct S3Config {
    pub bucket_name: String,
    pub endpoint_url: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub use_path_style: bool,
}

#[derive(Debug, Clone)]
pub struct Storage {
    client: Client,
    bucket_name: String,
}

impl Deref for Storage {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl Storage {
    pub async fn new(config: S3Config) -> Result<Self, error::InitError> {
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

        Self::setup_bucket(&client, &config.bucket_name).await?;

        Ok(Storage {
            client,
            bucket_name: config.bucket_name,
        })
    }

    pub async fn get_photo_url_by_id<S: AsRef<str>>(
        &self,
        photo_id: S,
        photo_quality: PhotoQuality,
    ) -> Result<String, error::StorageError> {
        let response = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(Self::format_id_with_quality(
                photo_id.as_ref(),
                photo_quality,
            ))
            .presigned(PresigningConfig::expires_in(Duration::from_secs(6000))?)
            .await?;

        let url = String::from(response.uri().to_string().split('?').next().unwrap());

        Ok(url)
    }

    pub async fn get_photo_bytes_by_id<S: AsRef<str>>(
        &self,
        photo_id: S,
        photo_quality: PhotoQuality,
    ) -> Result<Vec<u8>, error::StorageError> {
        let photo = self
            .get_object()
            .bucket(&self.bucket_name)
            .key(Self::format_id_with_quality(
                photo_id.as_ref(),
                photo_quality,
            ))
            .send()
            .await?;

        let bytes = photo.body.collect().await?;
        let bytes = bytes.to_vec();

        Ok(bytes)
    }

    pub async fn create_photo<S: AsRef<str>>(
        &self,
        photo_id: S,
        photo_quality: PhotoQuality,
        bytes: Vec<u8>,
    ) -> Result<(), error::StorageError> {
        let byte_stream = ByteStream::from(bytes);

        self.put_object()
            .bucket(&self.bucket_name)
            .key(Self::format_id_with_quality(
                photo_id.as_ref(),
                photo_quality,
            ))
            .body(byte_stream)
            .send()
            .await?;

        Ok(())
    }

    pub async fn delete_photo<S: AsRef<str>>(
        &self,
        photo_id: S,
        photo_quality: PhotoQuality,
    ) -> Result<(), error::StorageError> {
        self.delete_object()
            .bucket(&self.bucket_name)
            .key(Self::format_id_with_quality(
                photo_id.as_ref(),
                photo_quality,
            ))
            .send()
            .await?;

        Ok(())
    }

    async fn setup_bucket(
        client: &Client,
        bucket_name: &String,
    ) -> Result<(), SdkError<HeadBucketError>> {
        if let Err(err) = client.head_bucket().bucket(bucket_name).send().await {
            match err {
                SdkError::ServiceError(ref error) => {
                    // If the error is not found, a new bucket can be created, otherwise rethrow the error
                    if error.err().is_not_found() {
                        // Track how long the bucket creation takes
                        let start = Instant::now();

                        // Create the bucket
                        client
                            .create_bucket()
                            .bucket(bucket_name)
                            .send()
                            .await
                            .expect("failed to create new bucket");

                        // Update the bucket access policy
                        client
                            .put_bucket_policy()
                            .bucket(bucket_name)
                            .policy(format!(
                                r#"{{
                                    "Version": "2012-10-17",
                                    "Statement": [
                                        {{
                                            "Effect": "Allow",
                                            "Principal": {{
                                                "AWS": [
                                                    "*"
                                                ]
                                            }},
                                            "Action": [
                                                "s3:GetObject"
                                            ],
                                            "Resource": [
                                                "arn:aws:s3:::{}/*"
                                            ]
                                        }}
                                    ]
                                }}"#,
                                bucket_name
                            ))
                            .send()
                            .await
                            .expect("failed to set bucket policy");

                        // Stop the timer
                        let duration = start.elapsed();

                        // Log the duration of the bucket creation
                        info!(
                            "bucket '{}' did not exist yet; created new bucket (took {:?})",
                            bucket_name, duration
                        );
                    } else {
                        return Err(err);
                    }
                }
                _ => return Err(err),
            }
        }

        Ok(())
    }

    fn format_id_with_quality(photo_id: &str, photo_quality: PhotoQuality) -> String {
        format!("{}_{}", photo_id, photo_quality)
    }
}
