use std::ops::Deref;

use aws_credential_types::Credentials;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Config};
use aws_types::region::Region;
use tracing::instrument;

use crate::database::PhotoQuality;
use crate::storage_engine::error::StorageError;

pub mod aws_error {
    pub use aws_sdk_s3::error::*;
}

pub mod error {
    use aws_sdk_s3::error::{
        CreateBucketError, DeleteObjectError, GetObjectError, HeadBucketError,
        PutBucketPolicyError, PutObjectError,
    };
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
        #[error("{0}")]
        CreateBucket(#[from] SdkError<CreateBucketError>),
        #[error("{0}")]
        PutBucketPolicy(#[from] SdkError<PutBucketPolicyError>),
        #[error("{0}")]
        HeadBucket(#[from] SdkError<HeadBucketError>),
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
    pub async fn new(config: S3Config) -> Result<Self, StorageError> {
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
        photo_quality: &PhotoQuality,
    ) -> Result<String, error::StorageError> {
        let url = format!(
            "https://{}.s3.amazonaws.com/{}_{}",
            self.bucket_name,
            photo_id.as_ref(),
            photo_quality
        );
        //
        // let response = self
        //     .client
        //     .get_object()
        //     .bucket(&self.bucket_name)
        //     .key(Self::format_id_with_quality(
        //         photo_id.as_ref(),
        //         photo_quality,
        //     ))
        //     .send(PresigningConfig::builder().build()?)
        //     .await?;
        //
        // let url = String::from(response.uri().to_string().split('?').next().unwrap());

        Ok(url)
    }

    pub async fn get_photo_bytes_by_id<S: AsRef<str>>(
        &self,
        photo_id: S,
        photo_quality: &PhotoQuality,
    ) -> Result<Vec<u8>, StorageError> {
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
        photo_quality: &PhotoQuality,
        bytes: Vec<u8>,
    ) -> Result<(), StorageError> {
        let byte_stream = ByteStream::from(bytes);

        self.put_object()
            .bucket(&self.bucket_name)
            .key(Self::format_id_with_quality(
                photo_id.as_ref(),
                photo_quality,
            ))
            .body(byte_stream)
            .content_type("image/webp")
            .send()
            .await?;

        Ok(())
    }

    pub async fn delete_photo<S: AsRef<str>>(
        &self,
        photo_id: S,
        photo_quality: &PhotoQuality,
    ) -> Result<(), StorageError> {
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

    #[instrument(skip_all)]
    async fn setup_bucket(client: &Client, bucket_name: &String) -> Result<(), StorageError> {
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
            .await?;

        Ok(())
    }

    fn format_id_with_quality(photo_id: &str, photo_quality: &PhotoQuality) -> String {
        format!("{}_{}", photo_id, photo_quality)
    }
}
