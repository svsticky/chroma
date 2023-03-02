use crate::database::{Database, DbResult, Photo};
use rand::Rng;
use sqlx::FromRow;
use std::borrow::Cow;
use time::OffsetDateTime;

pub struct Album<'a> {
    db: &'a Database,
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub cover_photo_id: Option<String>,
}

#[derive(FromRow)]
struct _Album {
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub cover_photo_id: Option<String>,
}

impl _Album {
    fn to_album(self, db: &Database) -> Album {
        Album {
            db,
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            cover_photo_id: self.cover_photo_id,
        }
    }
}

impl<'a> From<Album<'a>> for proto::Album {
    fn from(x: Album<'a>) -> Self {
        Self {
            id: x.id,
            name: x.name,
            created_at: x.created_at,
            cover_photo_id: x.cover_photo_id,
        }
    }
}

impl<'a> Album<'a> {
    pub const MAX_NAME_LENGTH: usize = 64;
    pub const ID_PREFIX: &'static str = "ALB_";
    pub const MAX_ID_LEN: usize = 32;

    fn generate_id() -> String {
        let random: String = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(Self::MAX_ID_LEN - Self::ID_PREFIX.len())
            .map(char::from)
            .collect();
        format!("{}{random}", Self::ID_PREFIX)
    }

    pub async fn create(db: &'a Database, name: impl Into<Cow<'_, str>>) -> DbResult<Album<'a>> {
        let name = name.into();
        let id = Self::generate_id();
        let created_at = OffsetDateTime::now_utc().unix_timestamp();

        sqlx::query("INSERT INTO album_metadata (id, name, created_at) VALUES (?, ?, ?)")
            .bind(&id)
            .bind(&name)
            .bind(created_at)
            .execute(&**db)
            .await?;

        Ok(Self {
            db,
            id,
            name: name.to_string(),
            created_at,
            cover_photo_id: None,
        })
    }

    pub async fn get_by_id<S: AsRef<str> + Sync>(
        db: &'a Database,
        id: S,
    ) -> DbResult<Option<Album<'a>>> {
        let album: Option<_Album> = sqlx::query_as(
            "SELECT id, name, created_at, cover_photo_id FROM album_metadata WHERE id = ?",
        )
        .bind(id.as_ref())
        .fetch_optional(&**db)
        .await?;

        Ok(album.map(|x| x.to_album(db)))
    }

    pub async fn update_cover_photo(&mut self, cover_photo: &Photo<'_>) -> DbResult<()> {
        sqlx::query("UPDATE album_metadata SET cover_photo_id = ? WHERE id = ?")
            .bind(&cover_photo.id)
            .bind(&self.id)
            .execute(&**self.db)
            .await?;

        self.cover_photo_id = Some(cover_photo.id.clone());
        Ok(())
    }

    pub async fn update_name(&mut self, new_name: impl Into<Cow<'_, str>>) -> DbResult<()> {
        let new_name = new_name.into();
        sqlx::query("UPDATE album_metadata SET name = ? WHERE id = ?")
            .bind(&new_name)
            .bind(&self.id)
            .execute(&**self.db)
            .await?;
        self.name = new_name.to_string();
        Ok(())
    }

    pub async fn delete(self) -> DbResult<()> {
        let mut tx = self.db.begin().await?;

        // Must satisfy the foreign key constraint
        // So unset the cover photo before removing all photoss
        sqlx::query("UPDATE album_metadata SET cover_photo_id = NULL WHERE id = ?")
            .bind(&self.id)
            .execute(&mut tx)
            .await?;

        sqlx::query("DELETE FROM photo_metadata WHERE album_id = ?")
            .bind(&self.id)
            .execute(&mut tx)
            .await?;

        sqlx::query("DELETE FROM album_metadata WHERE id = ?")
            .bind(&self.id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn list(db: &'a Database) -> DbResult<Vec<Album<'a>>> {
        let selfs: Vec<_Album> =
            sqlx::query_as("SELECT id, name, created_at, cover_photo_id FROM album_metadata")
                .fetch_all(&**db)
                .await?;

        Ok(selfs.into_iter().map(|x| x.to_album(db)).collect())
    }
}
