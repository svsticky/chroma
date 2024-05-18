use std::borrow::Cow;
use std::fmt;
use std::fmt::Formatter;

<<<<<<< Updated upstream
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
=======
use rand::Rng;
use sqlx::FromRow;
use time::OffsetDateTime;
>>>>>>> Stashed changes

use crate::database::{Database, DatabaseResult, Photo, StandardUser, TokenUser, User, UserType};

#[derive(FromRow)]
struct AlbumRow {
    id: String,
    name: String,
    created_at: i64,
    cover_photo_id: Option<String>,
    published: bool,
    created_by: i32,
    created_by_type: UserType,
    published_by: Option<i32>,
    published_by_type: Option<UserType>,
    published_at: Option<i64>,
}

<<<<<<< Updated upstream
#[derive(Clone, Type)]
#[sqlx(type_name = "user_type")]
enum _UserType {
    Koala,
    Service,
}

impl _Album {
    fn into_album(self) -> Album {
=======
impl AlbumRow {
    pub async fn to_album(self, db: &Database) -> Album {
>>>>>>> Stashed changes
        Album {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            published: self.published,
            published_by: match (self.published_by_type, self.published_by) {
                (Some(UserType::Standard), Some(id)) => Some(User::Standard(
                    StandardUser::get_by_id(db, id).await.unwrap().unwrap(),
                )),
                (Some(UserType::Token), Some(id)) => Some(User::Token(
                    TokenUser::get_by_id(db, id).await.unwrap().unwrap(),
                )),
                _ => None,
            },
            published_at: self.published_at,
            created_by: match self.created_by_type {
                UserType::Standard => User::Standard(
                    StandardUser::get_by_id(db, self.created_by)
                        .await
                        .unwrap()
                        .unwrap(),
                ),
                UserType::Token => User::Token(
                    TokenUser::get_by_id(db, self.created_by)
                        .await
                        .unwrap()
                        .unwrap(),
                ),
            },
            cover_photo: match self.cover_photo_id {
                Some(id) => Photo::get_by_id(db, id).await.unwrap(),
                _ => None,
            },
        }
    }
}

<<<<<<< Updated upstream
impl Album {
=======
pub struct Album<'a> {
    db: &'a Database,
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub created_by: User<'a>,
    pub published: bool,
    pub published_by: Option<User<'a>>,
    pub published_at: Option<i64>,
    pub cover_photo: Option<Photo<'a>>,
}

// Manually impl debug as to not print the `db` field
impl fmt::Debug for Album<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Album")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("created_at", &self.created_at)
            .field("created_by", &self.created_by)
            .field("published", &self.published)
            .field("published_at", &self.published_at)
            .field("published_by", &self.published_by)
            .field("cover_photo", &self.cover_photo)
            .finish()
    }
}

impl<'a> Album<'a> {
>>>>>>> Stashed changes
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

<<<<<<< Updated upstream
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

=======
>>>>>>> Stashed changes
    pub async fn create(
        db: &Database,
        name: impl Into<Cow<'_, str>>,
<<<<<<< Updated upstream
        is_draft: bool,
        created_by: UserType,
    ) -> DbResult<Album> {
=======
        published: bool,
        created_by: User<'a>,
    ) -> DatabaseResult<Album<'a>> {
>>>>>>> Stashed changes
        let name = name.into();
        let id = Self::generate_id();
        let created_at = OffsetDateTime::now_utc().unix_timestamp();

        let (created_by_type, created_by_id) = match &created_by {
            User::Standard(user) => (UserType::Standard, user.koala_id),
            User::Token(user) => (UserType::Token, user.id),
        };

        // If something is a draft, there can be no publisher.
        // If an album at creation is a published one, then the publisher is also the creator.
        let published_at = published.then_some(created_at);
        let published_by_type = published.then_some(created_by_type);
        let published_by_id = published.then_some(created_by_id);

        sqlx::query(
            "INSERT INTO album_metadata \
                    (id, name, created_at, created_by, is_draft, published_by, published_at, published_by_type, created_by_type) \
                VALUES \
                    ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(&id)
            .bind(&name)
            .bind(created_at)
            .bind(created_by_id)
            .bind(published)
            .bind(published_by_id)
            .bind(published_at)
            .bind(published_by_type)
            .bind(created_by_type)
            .execute(&**db)
            .await?;

        Ok(Self {
            id,
            name: name.to_string(),
            published,
            published_by: published.then(|| created_by.clone()),
            published_at,
            created_at,
            created_by,
            cover_photo: None,
        })
    }

<<<<<<< Updated upstream
    pub async fn get_by_id<S: AsRef<str> + Sync>(db: &Database, id: S) -> DbResult<Option<Album>> {
        let album: Option<_Album> = sqlx::query_as("SELECT * FROM album_metadata WHERE id = $1")
=======
    pub async fn get_by_id<S: AsRef<str> + Sync>(
        db: &'a Database,
        id: S,
    ) -> DatabaseResult<Option<Album<'a>>> {
        let album: Option<AlbumRow> = sqlx::query_as("SELECT * FROM album_metadata WHERE id = $1")
>>>>>>> Stashed changes
            .bind(id.as_ref())
            .fetch_optional(&**db)
            .await?;

<<<<<<< Updated upstream
        Ok(album.map(|x| x.into_album()))
    }

    pub async fn update_cover_photo(
        &mut self,
        cover_photo: &Photo<'_>,
        db: &Database,
    ) -> DbResult<()> {
=======
        Ok(album.map(|x| x.to_album(db) ))
    }

    pub async fn update_cover_photo(&mut self, cover_photo: &Photo<'_>) -> DatabaseResult<()> {
>>>>>>> Stashed changes
        sqlx::query("UPDATE album_metadata SET cover_photo_id = $1 WHERE id = $2")
            .bind(&cover_photo.id)
            .bind(&self.id)
            .execute(&**db)
            .await?;

        self.cover_photo = Some(Photo::get_by_id(self.db, cover_photo.id.clone()));
        Ok(())
    }

<<<<<<< Updated upstream
    pub async fn update_name(
        &mut self,
        new_name: impl Into<Cow<'_, str>>,
        db: &Database,
    ) -> DbResult<()> {
=======
    pub async fn update_name(&mut self, new_name: impl Into<Cow<'_, str>>) -> DatabaseResult<()> {
>>>>>>> Stashed changes
        let new_name = new_name.into();
        sqlx::query("UPDATE album_metadata SET name = $1 WHERE id = $2")
            .bind(&new_name)
            .bind(&self.id)
            .execute(&**db)
            .await?;
        self.name = new_name.to_string();
        Ok(())
    }

<<<<<<< Updated upstream
    pub async fn set_published(&mut self, published_by: UserType, db: &Database) -> DbResult<()> {
=======
    pub async fn set_published(&mut self, published_by: &User<'a>) -> DatabaseResult<()> {
>>>>>>> Stashed changes
        let published_at = OffsetDateTime::now_utc().unix_timestamp();

        let (published_by_type, published_by_id) = match &published_by {
            User::Standard(user) => (*user.koala_id),
            User::Token(user) => (*user.id),
        };

        sqlx::query("UPDATE album_metadata SET published_by = $1, published_at = $2, published_by_type = $3, is_draft = false WHERE id = $4")
            .bind(published_by_id)
            .bind(published_at)
            .bind(published_by_type)
            .bind(&self.id)
            .execute(&**db)
            .await?;

        self.published = true;
        self.published_by = Some(published_by);
        self.published_at = Some(published_at);

        Ok(())
    }

<<<<<<< Updated upstream
    pub async fn set_draft(&mut self, db: &Database) -> DbResult<()> {
=======
    pub async fn set_draft(&mut self) -> DatabaseResult<()> {
>>>>>>> Stashed changes
        sqlx::query("UPDATE album_metadata SET published_by = NULL, published_by_type = NULL, published_at = NULL, is_draft = true WHERE id = $1")
            .bind(&self.id)
            .execute(&**db)
            .await?;

        self.published = false;
        self.published_by = None;
        self.published_at = None;

        Ok(())
    }

<<<<<<< Updated upstream
    pub async fn delete(self, db: &Database) -> DbResult<()> {
        let mut tx = db.begin().await?;
=======
    pub async fn delete(self) -> DatabaseResult<()> {
        let mut tx = self.db.begin().await?;
>>>>>>> Stashed changes

        // Must satisfy the foreign key constraint
        // So unset the cover photo before removing all photos
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

<<<<<<< Updated upstream
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
=======
    pub async fn list(db: &'a Database) -> DatabaseResult<Vec<Album<'a>>> {
        let selfs: Vec<AlbumRow> = sqlx::query_as("SELECT * FROM album_metadata")
            .fetch_all(&**db)
            .await?;

        Ok(selfs.into_iter().map(|x| x.to_album(db)).collect())
    }

    pub async fn to_proto(self) -> DatabaseResult<proto::Album> {
        Ok(proto::Album {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            cover_photo_id: self.cover_photo_id,
            published: self.published,
            created_by: match self.created_by {
                User::Standard(user) => Some(user.to_proto()),
                User::Token(user) => Some(user.to_proto()),
            },
            published_by: match self.published_by {
                Some(User::Standard(user)) => Some(user.to_proto()),
                Some(User::Token(user)) => Some(user.to_proto()),
                None => None,
            },
            published_at: self.published_at,
        })
>>>>>>> Stashed changes
    }
}
