use crate::s3::{S3Config, S3Error, S3InitError, S3};
use aws_sdk_s3::error::GetObjectErrorKind;
use aws_smithy_http::result::SdkError;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;
use strum_macros::Display;
use thiserror::Error;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::debug;

#[derive(Debug, Error)]
pub enum StorageEngineError {
    /// S3 Error
    #[error("{0}")]
    S3(#[from] S3Error),
    /// IO error
    #[error("{0}")]
    Io(#[from] io::Error),
}

/// Backend agnostic storage engine.
#[derive(Debug, Clone)]
pub struct StorageEngine(StorageEngineMode);

#[derive(Debug, Clone)]
enum StorageEngineMode {
    S3(S3),
    File(FileEngine),
}

impl StorageEngine {
    /// Create a storage engine backed by S3.
    ///
    /// # Errors
    ///
    /// If initializing the S3 engine failed.
    pub async fn new_s3(config: S3Config) -> Result<Self, S3InitError> {
        let sdk = S3::new(config).await?;

        Ok(Self(StorageEngineMode::S3(sdk)))
    }

    /// Create a storage engine backed by the local filesystem.
    ///
    /// # Errors
    ///
    /// If an IO error occurred.
    pub async fn new_file(base_path: PathBuf) -> Result<Self, io::Error> {
        if !base_path.exists() {
            debug!("`file_base` does not exist, creating.");
            tokio::fs::create_dir_all(&base_path).await?;
        }

        Ok(Self(StorageEngineMode::File(FileEngine { base_path })))
    }

    /// Retrieve a photo by ID.
    ///
    /// # Errors
    ///
    /// If the storage operation failed
    pub async fn get_photo_by_id<S: AsRef<str>>(
        &self,
        photo_id: S,
        quality_preference: PhotoQuality,
    ) -> Result<Vec<u8>, StorageEngineError> {
        let quality_id = fmt_id_with_quality(photo_id.as_ref(), &quality_preference);

        match &self.0 {
            StorageEngineMode::S3(s3) => match s3.get_photo_by_id(quality_id).await {
                Ok(photo) => Ok(photo),
                Err(S3Error::GetObject(e)) => match &e {
                    SdkError::ServiceError(svc_e) => match svc_e.err().kind {
                        GetObjectErrorKind::NoSuchKey(_) => {
                            if quality_preference == PhotoQuality::Original {
                                Err(StorageEngineError::S3(S3Error::GetObject(e)))
                            } else {
                                Ok(s3.get_photo_by_id(fmt_id_with_quality(photo_id.as_ref(), &PhotoQuality::Original)).await?)
                            }
                        }
                        _ => Err(StorageEngineError::S3(S3Error::GetObject(e))),
                    },
                    _ => Err(StorageEngineError::S3(S3Error::GetObject(e))),
                },
                Err(e) => Err(StorageEngineError::S3(e)),
            },
            StorageEngineMode::File(engine) => match engine.get_photo_by_id(quality_id).await {
                Ok(photo) => Ok(photo),
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => {
                        if quality_preference == PhotoQuality::Original {
                            Err(e.into())
                        } else {
                            Ok(engine.get_photo_by_id(fmt_id_with_quality(photo_id.as_ref(), &PhotoQuality::Original)).await?)
                        }
                    }
                    _ => Err(e.into()),
                },
            },
        }
    }

    /// Create a photo.
    ///
    /// # Errors
    ///
    /// If the storage operation failed
    pub async fn create_photo<S: AsRef<str>>(
        &self,
        photo_id: S,
        bytes: Vec<u8>,
        quality: PhotoQuality,
    ) -> Result<(), StorageEngineError> {
        let quality_id = fmt_id_with_quality(photo_id.as_ref(), &quality);
        Ok(match &self.0 {
            StorageEngineMode::S3(s3) => s3.create_photo(quality_id, bytes).await?,
            StorageEngineMode::File(engine) => engine.create_photo(quality_id, bytes).await?,
        })
    }

    /// Delete a photo by its ID.
    ///
    /// # Errors
    ///
    /// If the storage operation failed
    pub async fn delete_photo<S: AsRef<str>>(
        &self,
        photo_id: S,
        quality: PhotoQuality,
    ) -> Result<(), StorageEngineError> {
        let quality_id = fmt_id_with_quality(photo_id.as_ref(), &quality);
        Ok(match &self.0 {
            StorageEngineMode::S3(s3) => s3.delete_photo(quality_id).await?,
            StorageEngineMode::File(engine) => engine.delete_photo(quality_id).await?,
        })
    }
}

/// Storage engine backed by the local filesystem
#[derive(Debug, Clone)]
struct FileEngine {
    base_path: PathBuf,
}

impl FileEngine {
    async fn get_photo_by_id<S: AsRef<str>>(&self, photo_id: S) -> Result<Vec<u8>, io::Error> {
        let mut file = fs::File::open(self.base_path.join(photo_id.as_ref())).await?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await?;

        Ok(buf)
    }

    async fn create_photo<S: AsRef<str>>(
        &self,
        photo_id: S,
        bytes: Vec<u8>,
    ) -> Result<(), io::Error> {
        let mut file = fs::File::create(self.base_path.join(photo_id.as_ref())).await?;
        file.write_all(&bytes).await?;

        Ok(())
    }

    async fn delete_photo<S: AsRef<str>>(&self, photo_id: S) -> Result<(), io::Error> {
        let path = self.base_path.join(photo_id.as_ref());
        fs::remove_file(&path).await?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum PhotoQuality {
    Original,
    W400,
    W1600,
}

pub(crate) fn fmt_id_with_quality(id: &str, quality: &PhotoQuality) -> String {
    format!("{id}_{quality}")
}
