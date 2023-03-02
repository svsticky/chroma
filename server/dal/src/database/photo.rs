use rand::Rng;
use sqlx::FromRow;
use time::OffsetDateTime;
use crate::database::{Album, Database, DbResult};
use crate::s3::{S3, S3Error};

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
    pub fn to_photo(self, db: &Database) -> Photo {
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

    /// Convert a [Photo] to a [proto::Photo].
    /// Retrieves the photo's content from S3.
    ///
    /// # Errors
    ///
    /// If fetching the photo's contents from S3 failed
    pub async fn photo_to_proto(self, s3: &S3) -> Result<proto::Photo, S3Error> {
        let photo_bytes = s3.get_photo_by_id(&self.id).await?;
        Ok(proto::Photo {
            id: self.id,
            album_id: self.album_id,
            created_at: self.created_at,
            photo_data: photo_bytes
        })
    }

    fn generate_id() -> String {
        let random: String = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(Self::MAX_ID_LEN - Self::ID_PREFIX.len()).map(char::from).collect();
        format!("{}{random}", Self::ID_PREFIX)
    }

    pub async fn create(db: &'a Database, album: &Album<'_>) -> DbResult<Photo<'a>> {
        let id = Self::generate_id();
        let created_at = OffsetDateTime::now_utc().unix_timestamp();

        sqlx::query("INSERT INTO photo_metadata (id, album_id, created_at) VALUES (?, ?, ?)")
            .bind(&id)
            .bind(&album.id)
            .bind(created_at)
            .execute(&**db)
            .await?;

        Ok(Self {
            db,
            id,
            album_id: album.id.clone(),
            created_at
        })
    }

    pub async fn get_by_id<S: AsRef<str>>(db: &'a Database, id: S) -> DbResult<Option<Photo<'a>>> {
        let photo: Option<_Photo> = sqlx::query_as("SELECT id, album_id, created_at FROM photo_metadata WHERE id = ?")
            .bind(id.as_ref())
            .fetch_optional(&**db)
            .await?;

        Ok(photo.map(|photo| photo.to_photo(db)))
    }

    pub async fn delete(self) -> DbResult<()> {
        let mut tx = self.db.begin().await?;
        // Remove the photo from the album cover
        sqlx::query("UPDATE album_metadata SET cover_photo_id = NULL WHERE id = ? AND cover_photo_id = ?")
            .bind(&self.album_id)
            .bind(&self.id)
            .execute(&mut tx)
            .await?;

        // Remove the photo metadata
        sqlx::query("DELETE FROM photo_metadata WHERE id = ?")
            .bind(&self.id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn list(db: &'a Database) -> DbResult<Vec<Photo<'a>>> {
        let selfs: Vec<_Photo> = sqlx::query_as("SELECT id, album_id, created_at FROM photo_metadata")
            .fetch_all(&**db)
            .await?;
        Ok(selfs.into_iter().map(|photo| photo.to_photo(db)).collect())
    }

    pub async fn list_in_album<S: AsRef<str>>(db: &'a Database, album_id: S) -> DbResult<Vec<Photo<'a>>> {
        let selfs: Vec<_Photo> = sqlx::query_as("SELECT id, album_id, created_at FROM photo_metadata WHERE album_id = ?")
            .bind(album_id.as_ref())
            .fetch_all(&**db)
            .await?;
        Ok(selfs.into_iter().map(|photo| photo.to_photo(db)).collect())
    }
}
