use rand::Rng;
use sqlx::{FromRow, Type};
use strum_macros::Display;

use proto::photo_respone::Response;
use proto::PhotoRespone;

use crate::database::{Album, Database, DbResult};
use crate::storage_engine::Storage;
use crate::DalError;

#[derive(Clone)]
pub struct Photo<'a> {
    db: &'a Database,
    pub id: String,
    pub album_id: String,
    pub created_at: i64,
}

#[derive(FromRow)]
struct _Photo {
    pub id: String,
    pub album_id: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Type, Display, PartialEq, Eq)]
#[sqlx(type_name = "photo_quality")]
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

#[derive(FromRow, Debug)]
pub struct PhotoS3Url {
    pub photo_id: String,
    pub s3_url: String,
    pub quality: PhotoQuality,
}

impl _Photo {
    pub fn into_photo(self, db: &Database) -> Photo {
        Photo {
            db,
            id: self.id,
            album_id: self.album_id,
            created_at: self.created_at,
        }
    }
}

impl<'a> Photo<'a> {
    pub const ID_PREFIX: &'static str = "PH_";
    pub const MAX_ID_LEN: usize = 32;

    pub async fn photo_to_proto_url(
        &self,
        storage: &Storage,
        quality_preference: &PhotoQuality,
    ) -> Result<proto::Photo, DalError> {
        let has_pref = self.is_quality_created(quality_preference).await?;
        let quality = if has_pref {
            quality_preference.clone()
        } else {
            PhotoQuality::Original
        };

        // Check if we already have a URL for the picture
        let url =
            if let Some(s3_url) = PhotoS3Url::get_for_photo(self.db, &self.id, &quality).await? {
                s3_url.s3_url
            } else {
                let url = storage.get_photo_url_by_id(&self.id, &quality).await?;
                let _ = PhotoS3Url::new(self.db, self.id.clone(), url.clone(), quality).await;
                url
            };

        Ok(proto::Photo {
            id: self.id.clone(),
            album_id: self.album_id.clone(),
            created_at: self.created_at,
            data_type: proto::PhotoResponseType::Url as i32,
            data: Some(PhotoRespone {
                response: Some(Response::Url(url)),
            }),
        })
    }

    /// Convert a [Photo] to a [proto::Photo].
    /// Retrieves the photo's content from S3.
    ///
    /// # Errors
    ///
    pub async fn photo_to_proto_bytes(
        self,
        storage: &Storage,
        quality_preference: PhotoQuality,
    ) -> Result<proto::Photo, DalError> {
        let has_pref = self.is_quality_created(&quality_preference).await?;
        let quality = if has_pref {
            quality_preference
        } else {
            PhotoQuality::Original
        };

        let photo_bytes = storage.get_photo_bytes_by_id(&self.id, &quality).await?;
        Ok(proto::Photo {
            id: self.id,
            album_id: self.album_id,
            created_at: self.created_at,
            data_type: proto::PhotoResponseType::InResponse as i32,
            data: Some(PhotoRespone {
                response: Some(Response::Bytes(photo_bytes)),
            }),
        })
    }

    fn generate_id() -> String {
        let random: String = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(Self::MAX_ID_LEN - Self::ID_PREFIX.len())
            .map(char::from)
            .collect();
        format!("{}{random}", Self::ID_PREFIX)
    }

    pub async fn create(db: &'a Database, album: &Album, created_at: i64) -> DbResult<Photo<'a>> {
        let id = Self::generate_id();

        sqlx::query("INSERT INTO photo_metadata (id, album_id, created_at) VALUES ($1, $2, $3)")
            .bind(&id)
            .bind(&album.id)
            .bind(created_at)
            .execute(&**db)
            .await?;

        Ok(Self {
            db,
            id,
            album_id: album.id.clone(),
            created_at,
        })
    }

    pub async fn get_by_id<S: AsRef<str>>(db: &'a Database, id: S) -> DbResult<Option<Photo<'a>>> {
        let photo: Option<_Photo> =
            sqlx::query_as("SELECT id, album_id, created_at FROM photo_metadata WHERE id = $1")
                .bind(id.as_ref())
                .fetch_optional(&**db)
                .await?;

        Ok(photo.map(|photo| photo.into_photo(db)))
    }

    pub async fn delete(self) -> DbResult<()> {
        let mut tx = self.db.begin().await?;
        // Remove the photo from the album cover
        sqlx::query(
            "UPDATE album_metadata SET cover_photo_id = NULL WHERE id = $1 AND cover_photo_id = $2",
        )
        .bind(&self.album_id)
        .bind(&self.id)
        .execute(&mut tx)
        .await?;

        // Remove the photo metadata
        sqlx::query("DELETE FROM photo_metadata WHERE id = $1")
            .bind(&self.id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn list(db: &'a Database) -> DbResult<Vec<Photo<'a>>> {
        let selfs: Vec<_Photo> =
            sqlx::query_as("SELECT id, album_id, created_at FROM photo_metadata")
                .fetch_all(&**db)
                .await?;
        Ok(selfs
            .into_iter()
            .map(|photo| photo.into_photo(db))
            .collect())
    }

    pub async fn list_in_album<S: AsRef<str>>(
        db: &'a Database,
        album_id: S,
    ) -> DbResult<Vec<Photo<'a>>> {
        let selfs: Vec<_Photo> = sqlx::query_as(
            "SELECT id, album_id, created_at FROM photo_metadata WHERE album_id = $1",
        )
        .bind(album_id.as_ref())
        .fetch_all(&**db)
        .await?;
        Ok(selfs
            .into_iter()
            .map(|photo| photo.into_photo(db))
            .collect())
    }

    /// Check whether an image quality has been created yet.
    ///
    /// # Errors
    ///
    /// If a database error occurs
    pub async fn is_quality_created(&self, quality: &PhotoQuality) -> DbResult<bool> {
        PhotoS3Url::get_for_photo(self.db, &self.id, quality)
            .await
            .map(|maybe_url| maybe_url.is_some())
    }
}

impl PhotoS3Url {
    pub async fn new(
        driver: &Database,
        photo_id: String,
        s3_url: String,
        photo_quality: PhotoQuality,
    ) -> DbResult<Self> {
        sqlx::query("INSERT INTO photo_s3_urls (photo_id, s3_url, quality) VALUES ($1, $2, $3)")
            .bind(&photo_id)
            .bind(&s3_url)
            .bind(&photo_quality)
            .execute(&**driver)
            .await?;

        Ok(Self {
            photo_id,
            s3_url,
            quality: photo_quality,
        })
    }

    pub async fn get_for_photo(
        driver: &Database,
        photo_id: &str,
        quality: &PhotoQuality,
    ) -> DbResult<Option<Self>> {
        sqlx::query_as("SELECT * FROM photo_s3_urls WHERE photo_id = $1 AND quality = $2")
            .bind(photo_id)
            .bind(quality)
            .fetch_optional(&**driver)
            .await
    }
}
