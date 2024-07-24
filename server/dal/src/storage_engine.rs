use std::ops::Deref;

use aws_credential_types::Credentials;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Config};
use aws_smithy_http::result::SdkError;
use aws_types::region::Region;
use tracing::{info, instrument};

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
    pub enum StorageError {
        #[error("couldn't retrieve bucket information ({0})")]
        HeadBucket(#[from] SdkError<HeadBucketError>),
        #[error("couldn't create bucket ({0})")]
        CreateBucket(#[from] SdkError<CreateBucketError>),
        #[error("couldn't set bucket policy ({0})")]
        PutBucketPolicy(#[from] SdkError<PutBucketPolicyError>),
        #[error("couldn't retrieve object ({0})")]
        GetObject(#[from] SdkError<GetObjectError>),
        #[error("couldn't upload object ({0})")]
        PutObject(#[from] SdkError<PutObjectError>),
        #[error("couldn't delete object ({0})")]
        DeleteObject(#[from] SdkError<DeleteObjectError>),
        #[error("couldn't to convert ByteStream ({0})")]
        ByteStream(#[from] aws_smithy_http::byte_stream::error::Error),
        #[error("couldn't create presigning config ({0})")]
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
    pub create_bucket: bool,
}

#[derive(Debug, Clone)]
pub struct Storage {
    client: Client,
    bucket_name: String,
    use_path_style: bool,
    endpoint_url: String,
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
                .endpoint_url(config.endpoint_url.clone())
                .region(Some(Region::new(config.region)))
                .credentials_provider(Credentials::from_keys(
                    config.access_key_id,
                    config.secret_access_key,
                    None,
                ))
                .build(),
        );

        if config.create_bucket && !Self::bucket_exists(&client, &config.bucket_name).await? {
            Self::create_bucket(&client, &config.bucket_name).await?;
        }

        Self::set_bucket_policy(&client, &config.bucket_name).await?;

        Ok(Storage {
            client,
            bucket_name: config.bucket_name,
            endpoint_url: config.endpoint_url,
            use_path_style: config.use_path_style,
        })
    }

    pub async fn get_photo_url_by_id<S: AsRef<str>>(
        &self,
        photo_id: S,
        photo_quality: &PhotoQuality,
    ) -> Result<String, StorageError> {
        let qstring = Self::format_id_with_quality(photo_id.as_ref(), photo_quality);
        let url = if self.use_path_style {
            format!("{}/{}/{}", self.endpoint_url, self.bucket_name, qstring)
        } else {
            format!("{}/{}", self.endpoint_url, qstring)
        };

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

    async fn create_bucket(client: &Client, bucket_name: &String) -> Result<(), StorageError> {
        client.create_bucket().bucket(bucket_name).send().await?;
        Ok(())
    }

    async fn bucket_exists(client: &Client, bucket_name: &String) -> Result<bool, StorageError> {
        match client.head_bucket().bucket(bucket_name).send().await {
            Ok(_) => Ok(true),
            Err(SdkError::ServiceError(e)) if e.err().is_not_found() => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(skip_all)]
    async fn set_bucket_policy(client: &Client, bucket_name: &String) -> Result<(), StorageError> {
        info!("Setting bucket policy");

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
