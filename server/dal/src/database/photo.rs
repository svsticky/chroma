use crate::database::{Album, Database, DbResult};
use crate::storage_engine::{PhotoQuality, Storage};
use crate::DalError;
use proto::photo_respone::Response;
use proto::PhotoRespone;
use rand::Rng;
use sqlx::FromRow;

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
        self,
        storage: &Storage,
        quality_preference: PhotoQuality,
    ) -> Result<proto::Photo, DalError> {
        let has_pref = self.is_quality_created(quality_preference.clone()).await?;
        let quality = if has_pref {
            quality_preference
        } else {
            PhotoQuality::Original
        };

        let url = storage
            .get_photo_url_by_id( &self.id, quality)
            .await?;

        Ok(proto::Photo {
            id: self.id,
            album_id: self.album_id,
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
        let has_pref = self.is_quality_created(quality_preference.clone()).await?;
        let quality = if has_pref {
            quality_preference
        } else {
            PhotoQuality::Original
        };

        let photo_bytes = storage
            .get_photo_bytes_by_id(&self.id, quality)
            .await?;
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
    /// This will always return true for [PhotoQuality::Original].
    ///
    /// # Errors
    ///
    /// If a database error occurs
    pub async fn is_quality_created(&self, quality: PhotoQuality) -> DbResult<bool> {
        match quality {
            PhotoQuality::Original => Ok(true),
            PhotoQuality::W400 => self.is_quality_w400_created().await,
            PhotoQuality::W1600 => self.is_quality_w1600_created().await,
        }
    }

    async fn is_quality_w400_created(&self) -> DbResult<bool> {
        let value: bool =
            sqlx::query_scalar("SELECT w400_created FROM photo_metadata WHERE id = $1")
                .bind(&self.id)
                .fetch_one(&**self.db)
                .await?;
        Ok(value)
    }

    async fn is_quality_w1600_created(&self) -> DbResult<bool> {
        let value: bool =
            sqlx::query_scalar("SELECT w1600_created FROM photo_metadata WHERE id = $1")
                .bind(&self.id)
                .fetch_one(&**self.db)
                .await?;
        Ok(value)
    }

    /// Set whether an image quality has been created or not.
    /// This is a no-op for [PhotoQuality::Original].
    ///
    /// # Errors
    ///
    /// If a database error occurs
    pub async fn set_quality_created(&self, quality: PhotoQuality, created: bool) -> DbResult<()> {
        match quality {
            PhotoQuality::Original => Ok(()),
            PhotoQuality::W400 => self.set_quality_w400_created(created).await,
            PhotoQuality::W1600 => self.set_quality_w1600_created(created).await,
        }
    }

    async fn set_quality_w400_created(&self, created: bool) -> DbResult<()> {
        sqlx::query("UPDATE photo_metadata SET w400_created = $1 WHERE id = $2")
            .bind(created)
            .bind(&self.id)
            .execute(&**self.db)
            .await?;
        Ok(())
    }

    async fn set_quality_w1600_created(&self, created: bool) -> DbResult<()> {
        sqlx::query("UPDATE photo_metadata SET w1600_created = $1 WHERE id = $2")
            .bind(created)
            .bind(&self.id)
            .execute(&**self.db)
            .await?;
        Ok(())
    }
}
