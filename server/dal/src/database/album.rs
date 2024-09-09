use std::borrow::Cow;

use futures::future::{join_all, OptionFuture};
use rand::Rng;
use sqlx::{FromRow, Postgres, Type};
use time::OffsetDateTime;

use crate::database::{Database, DbResult, Photo};

#[derive(Clone, Debug)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub cover_photo: Option<Photo>,
    pub created_at: i64,
    pub created_by: UserType,
    pub published: bool,
    pub published_at: Option<i64>,
    pub published_by: Option<UserType>,
}

#[derive(Clone, Debug)]
pub enum UserType {
    Koala(i32),
    ServiceToken(i32),
}

#[derive(FromRow)]
struct AlbumRow {
    id: String,
    name: String,
    cover_photo_id: Option<String>,
    created_at: i64,
    created_by: i32,
    published: bool,
    published_at: Option<i64>,
    published_by: Option<i32>,
}

#[derive(Clone, Type)]
#[sqlx(type_name = "user_type")]
enum _UserType {
    Koala,
    Service,
}

impl Album {
    pub const MAX_NAME_LENGTH: usize = 64;
    pub const ID_PREFIX: &'static str = "ALB_";
    pub const MAX_ID_LEN: usize = 32;

    fn new(album_row: AlbumRow, cover_photo: Option<Photo>) -> Album {
        Album {
            id: album_row.id,
            name: album_row.name,
            cover_photo,
            created_at: album_row.created_at,
            created_by: UserType::Koala(album_row.created_by),
            published: album_row.published,
            published_by: album_row.published_by.map(UserType::Koala),
            published_at: album_row.published_at,
        }
    }

    fn generate_id() -> String {
        let random: String = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(Self::MAX_ID_LEN - Self::ID_PREFIX.len())
            .map(char::from)
            .collect();
        format!("{}{random}", Self::ID_PREFIX)
    }

    pub async fn create(
        db: &Database,
        name: impl Into<Cow<'_, str>>,
        published: bool,
        created_by: UserType,
    ) -> DbResult<Album> {
        let name = name.into();
        let id = Self::generate_id();

        let (_created_by_type, created_by_id) = match &created_by {
            UserType::Koala(id) => (_UserType::Koala, *id),
            UserType::ServiceToken(id) => (_UserType::Service, *id),
        };

        // If something is a draft, there can be no publisher.
        // If an album at creation is a published one, then the publisher is also the creator.
        let published_at = (!published).then_some(OffsetDateTime::now_utc().unix_timestamp());
        let published_by_id = (!published).then_some(match &created_by {
            UserType::Koala(id) => *id,
            UserType::ServiceToken(id) => *id,
        });

        Ok(Self::new(
            sqlx::query_as::<Postgres, AlbumRow>(
                "INSERT INTO albums \
                    (id, name, created_by, published, published_by, published_at) \
                VALUES \
                    ($1, $2, $3, $4, $5, $6) \
                RETURNING \
                    *",
            )
            .bind(&id)
            .bind(&name)
            .bind(created_by_id)
            .bind(published)
            .bind(published_by_id)
            .bind(published_at)
            .fetch_one(&**db)
            .await?,
            None,
        ))
    }

    pub async fn get_by_id(db: &Database, id: &str) -> DbResult<Option<Album>> {
        OptionFuture::from(
            sqlx::query_as::<Postgres, AlbumRow>("SELECT * FROM albums WHERE id = $1")
                .bind(id)
                .fetch_optional(&**db)
                .await?
                .map(|album_row| async {
                    let cover_photo = match &album_row.cover_photo_id {
                        Some(id) => Photo::get(db, id).await?,
                        None => None,
                    };

                    Ok(Self::new(album_row, cover_photo))
                }),
        )
        .await
        .map_or(Ok(None), |v| v.map(Some))
    }

    pub async fn list(db: &Database, published_only: bool) -> DbResult<Vec<Album>> {
        join_all(
            sqlx::query_as::<Postgres, AlbumRow>(if published_only {
                "SELECT * FROM albums WHERE published"
            } else {
                "SELECT * FROM albums"
            })
            .fetch_all(&**db)
            .await?
            .into_iter()
            .map(|album| async {
                let photo = match &album.cover_photo_id {
                    Some(id) => Photo::get(db, id).await?,
                    None => None,
                };

                Ok(Self::new(album, photo))
            }),
        )
        .await
        .into_iter()
        .collect::<DbResult<Vec<_>>>()
    }

    pub async fn update_cover_photo(
        &mut self,
        cover_photo: Option<Photo>,
        db: &Database,
    ) -> DbResult<()> {
        sqlx::query("UPDATE albums SET cover_photo_id = $1 WHERE id = $2")
            .bind(cover_photo.as_ref().map(|photo| &photo.id))
            .bind(&self.id)
            .execute(&**db)
            .await?;

        self.cover_photo = cover_photo;

        Ok(())
    }

    pub async fn update_name(&mut self, name: String, db: &Database) -> DbResult<()> {
        sqlx::query("UPDATE albums SET name = $1 WHERE id = $2")
            .bind(&name)
            .bind(&self.id)
            .execute(&**db)
            .await?;

        self.name = name;

        Ok(())
    }

    pub async fn set_published(&mut self, published_by: UserType, db: &Database) -> DbResult<()> {
        let published_at = OffsetDateTime::now_utc().unix_timestamp();

        let (_published_by_type, published_by_id) = match &published_by {
            UserType::Koala(id) => (_UserType::Koala, *id),
            UserType::ServiceToken(id) => (_UserType::Service, *id),
        };

        sqlx::query(
            "UPDATE albums SET published_by = $1, published_at = $2, published = true WHERE id = $3",
        )
            .bind(published_by_id)
            .bind(published_at)
            .bind(&self.id)
            .execute(&**db)
            .await?;

        self.published = true;
        self.published_by = Some(published_by);
        self.published_at = Some(published_at);

        Ok(())
    }

    pub async fn set_draft(&mut self, db: &Database) -> DbResult<()> {
        sqlx::query("UPDATE albums SET published_by = NULL, published_at = NULL, published = false WHERE id = $1")
            .bind(&self.id)
            .execute(&**db)
            .await?;

        self.published = false;
        self.published_by = None;
        self.published_at = None;

        Ok(())
    }

    pub async fn delete(self, db: &Database) -> DbResult<()> {
        sqlx::query("DELETE FROM albums WHERE id = $1")
            .bind(&self.id)
            .execute(&**db)
            .await?;

        Ok(())
    }
}

impl From<Album> for proto::Album {
    fn from(album: Album) -> proto::Album {
        proto::Album {
            id: Some(album.id),
            name: Some(album.name),
            cover_photo: album.cover_photo.map(|photo| photo.into()),
            created_at: Some(album.created_at),
            created_by: None, // Todo: Obtain users
            published: Some(album.published),
            published_at: album.published_at,
            published_by: None,
        }
    }
}
