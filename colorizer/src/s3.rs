use aws_credential_types::Credentials;
use aws_sdk_s3::{Client, Config};
use aws_sdk_s3::error::GetObjectError;
use aws_smithy_http::result::SdkError;
use aws_types::region::Region;
use thiserror::Error;

#[derive(Debug)]
pub struct S3 {
    client: Client,
    bucket_name: String,
}

#[derive(Debug, Error)]
pub enum S3InitError {
    #[error("Invalid App name provided")]
    AppName(#[from] aws_types::app_name::InvalidAppName),
}

#[derive(Debug, Error)]
pub enum S3Error {
    #[error("Failed to retrieve Object: {0}")]
    GetObject(#[from] SdkError<GetObjectError>),
    #[error("Failed to convert ByteStream: {0}")]
    ByteStream(#[from] aws_smithy_http::byte_stream::error::Error),
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
    pub fn new(config: S3Config) -> Result<S3, S3InitError> {
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

    pub async fn get_file<S: AsRef<str>>(&self, name: S) -> Result<Vec<u8>, S3Error> {
        let file = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(name.as_ref())
            .send()
            .await?;

        let bytes = file.body.collect().await?;

        Ok(bytes.to_vec())
    }
}