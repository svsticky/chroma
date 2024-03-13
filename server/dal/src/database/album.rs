use crate::database::{Database, DatabaseError, DbResult, Photo, User};
use rand::Rng;
use sqlx::{FromRow, Type};
use std::borrow::Cow;
use std::fmt;
use std::fmt::Formatter;
use time::OffsetDateTime;

#[derive(Clone)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub cover_photo_id: Option<String>,
    pub is_draft: bool,
    pub created_by: UserType,
    pub published_by: Option<UserType>,
    pub published_at: Option<i64>,
}

// Manually impl debug as to not print the `db` field
impl fmt::Debug for Album {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Album")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("created_at", &self.created_at)
            .field("created_by", &self.created_by)
            .field("published_at", &self.published_at)
            .field("published_by", &self.published_by)
            .field("is_draft", &self.is_draft)
            .field("cover_photo_id", &self.cover_photo_id)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub enum UserType {
    Koala(i32),
    ServiceToken(i32),
}

#[derive(FromRow)]
struct _Album {
    id: String,
    name: String,
    created_at: i64,
    cover_photo_id: Option<String>,
    is_draft: bool,
    created_by: i32,
    created_by_type: _UserType,
    published_by: Option<i32>,
    published_by_type: Option<_UserType>,
    published_at: Option<i64>,
}

#[derive(Clone, Type)]
#[sqlx(type_name = "user_type")]
enum _UserType {
    Koala,
    Service,
}

impl _Album {
    fn into_album(self) -> Album {
        Album {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            cover_photo_id: self.cover_photo_id,
            is_draft: self.is_draft,
            published_by: match (self.published_by_type, self.published_by) {
                (Some(_UserType::Koala), Some(id)) => Some(UserType::Koala(id)),
                (Some(_UserType::Service), Some(id)) => Some(UserType::ServiceToken(id)),
                _ => None,
            },
            published_at: self.published_at,
            created_by: match self.created_by_type {
                _UserType::Koala => UserType::Koala(self.created_by),
                _UserType::Service => UserType::ServiceToken(self.created_by),
            },
        }
    }
}

impl Album {
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

    async fn user_type_to_proto(db: &Database, user: UserType) -> DbResult<proto::AlbumUser> {
        Ok(match user {
            UserType::Koala(id) => {
                let user = User::get_by_id(db, id)
                    .await?
                    .ok_or(DatabaseError::RowNotFound)?;
                proto::AlbumUser {
                    id,
                    name: Some(user.name),
                    r#type: proto::UserType::Koala as i32,
                }
            }
            UserType::ServiceToken(id) => proto::AlbumUser {
                id,
                name: None,
                r#type: proto::UserType::Service as i32,
            },
        })
    }

    pub async fn to_proto(self, db: &Database) -> DbResult<proto::Album> {
        Ok(proto::Album {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            cover_photo_id: self.cover_photo_id,
            is_draft: self.is_draft,
            created_by: Some(Self::user_type_to_proto(db, self.created_by).await?),
            published_by: match self.published_by {
                Some(published_by) => Some(Self::user_type_to_proto(db, published_by).await?),
                None => None,
            },
            published_at: self.published_at,
        })
    }

    pub async fn create(
        db: &Database,
        name: impl Into<Cow<'_, str>>,
        is_draft: bool,
        created_by: UserType,
    ) -> DbResult<Album> {
        let name = name.into();
        let id = Self::generate_id();
        let created_at = OffsetDateTime::now_utc().unix_timestamp();

        let (created_by_type, created_by_id) = match &created_by {
            UserType::Koala(id) => (_UserType::Koala, *id),
            UserType::ServiceToken(id) => (_UserType::Service, *id),
        };

        // If something is a draft, there can be no publisher.
        // If an album at creation is a published one, then the publisher is also the creator.
        let published_at = (!is_draft).then_some(created_at);
        let published_by_type = (!is_draft).then_some(created_by_type.clone());
        let published_by_id = (!is_draft).then_some(match &created_by {
            UserType::Koala(id) => *id,
            UserType::ServiceToken(id) => *id,
        });

        sqlx::query(
            "INSERT INTO album_metadata \
                    (id, name, created_at, created_by, is_draft, published_by, published_at, published_by_type, created_by_type) \
                VALUES \
                    ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(&id)
            .bind(&name)
            .bind(created_at)
            .bind(created_by_id)
            .bind(is_draft)
            .bind(published_by_id)
            .bind(published_at)
            .bind(published_by_type)
            .bind(created_by_type)
            .execute(&**db)
            .await?;

        Ok(Self {
            id,
            name: name.to_string(),
            created_at,
            cover_photo_id: None,
            is_draft,
            published_by: (!is_draft).then(|| created_by.clone()),
            created_by,
            published_at,
        })
    }

    pub async fn get_by_id<S: AsRef<str> + Sync>(db: &Database, id: S) -> DbResult<Option<Album>> {
        let album: Option<_Album> = sqlx::query_as("SELECT * FROM album_metadata WHERE id = $1")
            .bind(id.as_ref())
            .fetch_optional(&**db)
            .await?;

        Ok(album.map(|x| x.into_album()))
    }

    pub async fn update_cover_photo(
        &mut self,
        cover_photo: &Photo<'_>,
        db: &Database,
    ) -> DbResult<()> {
        sqlx::query("UPDATE album_metadata SET cover_photo_id = $1 WHERE id = $2")
            .bind(&cover_photo.id)
            .bind(&self.id)
            .execute(&**db)
            .await?;

        self.cover_photo_id = Some(cover_photo.id.clone());
        Ok(())
    }

    pub async fn update_name(
        &mut self,
        new_name: impl Into<Cow<'_, str>>,
        db: &Database,
    ) -> DbResult<()> {
        let new_name = new_name.into();
        sqlx::query("UPDATE album_metadata SET name = $1 WHERE id = $2")
            .bind(&new_name)
            .bind(&self.id)
            .execute(&**db)
            .await?;
        self.name = new_name.to_string();
        Ok(())
    }

    pub async fn set_published(&mut self, published_by: UserType, db: &Database) -> DbResult<()> {
        let published_at = OffsetDateTime::now_utc().unix_timestamp();

        let (published_by_type, published_by_id) = match &published_by {
            UserType::Koala(id) => (_UserType::Koala, *id),
            UserType::ServiceToken(id) => (_UserType::Service, *id),
        };

        sqlx::query("UPDATE album_metadata SET published_by = $1, published_at = $2, published_by_type = $3, is_draft = false WHERE id = $4")
            .bind(published_by_id)
            .bind(published_at)
            .bind(published_by_type)
            .bind(&self.id)
            .execute(&**db)
            .await?;

        self.is_draft = false;
        self.published_by = Some(published_by);
        self.published_at = Some(published_at);

        Ok(())
    }

    pub async fn set_draft(&mut self, db: &Database) -> DbResult<()> {
        sqlx::query("UPDATE album_metadata SET published_by = NULL, published_by_type = NULL, published_at = NULL, is_draft = true WHERE id = $1")
            .bind(&self.id)
            .execute(&**db)
            .await?;

        self.is_draft = true;
        self.published_by = None;
        self.published_at = None;

        Ok(())
    }

    pub async fn delete(self, db: &Database) -> DbResult<()> {
        let mut tx = db.begin().await?;

        // Must satisfy the foreign key constraint
        // So unset the cover photo before removing all photoss
        sqlx::query("UPDATE album_metadata SET cover_photo_id = NULL WHERE id = $1")
            .bind(&self.id)
            .execute(&mut tx)
            .await?;

        sqlx::query("DELETE FROM photo_metadata WHERE album_id = $1")
            .bind(&self.id)
            .execute(&mut tx)
            .await?;

        sqlx::query("DELETE FROM album_metadata WHERE id = $1")
            .bind(&self.id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn list_ids(db: &Database) -> DbResult<Vec<String>> {
        Ok(sqlx::query_scalar("SELECT id FROM album_metadata")
            .fetch_all(&**db)
            .await?)
    }

    pub async fn list(db: &Database) -> DbResult<Vec<Album>> {
        let selfs: Vec<_Album> = sqlx::query_as("SELECT * FROM album_metadata")
            .fetch_all(&**db)
            .await?;

        Ok(selfs.into_iter().map(|x| x.into_album()).collect())
    }
}
