use crate::database::photo_album::PhotoAlbum;
use crate::database::photo_exif::PhotoExif;
use crate::database::photo_url::PhotoUrl;
use crate::database::{Database, DbResult};
use futures::future::{join_all, OptionFuture};
use rand::Rng;
use sqlx::{FromRow, Postgres, QueryBuilder};

#[derive(Debug, Clone)]
pub struct Photo {
    pub id: String,
    pub hash: Option<String>,
    pub uploaded_at: i64,
    pub uploaded_by: i32,
    pub captured_at: Option<i64>,
    pub linked_albums: Vec<PhotoAlbum>,
    pub media_urls: Vec<PhotoUrl>,
    pub exif_fields: Vec<PhotoExif>,
}

#[derive(FromRow)]
pub struct PhotoRow {
    pub id: String,
    pub hash: Option<String>,
    pub uploaded_at: i64,
    pub uploaded_by: i32,
    pub captured_at: Option<i64>,
}

impl Photo {
    pub const ID_PREFIX: &'static str = "PH_";
    pub const MAX_ID_LEN: usize = 32;

    fn generate_id() -> String {
        let random: String = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(Self::MAX_ID_LEN - Self::ID_PREFIX.len())
            .map(char::from)
            .collect();
        format!("{}{random}", Self::ID_PREFIX)
    }

    fn new(
        photo_row: PhotoRow,
        linked_albums: Vec<PhotoAlbum>,
        media_urls: Vec<PhotoUrl>,
        exif_fields: Vec<PhotoExif>,
    ) -> Photo {
        Photo {
            id: photo_row.id,
            hash: photo_row.hash,
            uploaded_at: photo_row.uploaded_at,
            uploaded_by: photo_row.uploaded_by,
            captured_at: photo_row.captured_at,
            linked_albums,
            media_urls,
            exif_fields,
        }
    }

    pub async fn create(db: &Database, hash: String, captured_at: Option<i64>) -> DbResult<Photo> {
        let id = Self::generate_id();

        let photo_row = sqlx::query_as::<Postgres, PhotoRow>(
            "INSERT INTO photos \
                (id, hash, uploaded_by, captured_at) \
            VALUES \
                ($1, $2, 1, $3) \
            RETURNING \
                id, hash, uploaded_at, uploaded_by, captured_at",
        )
        .bind(id)
        .bind(hash)
        .bind(captured_at)
        .fetch_one(&**db)
        .await?;

        Ok(Self::new(photo_row, vec![], vec![], vec![]))
    }

    pub async fn get(db: &Database, id: &str) -> DbResult<Option<Photo>> {
        OptionFuture::from(
            sqlx::query_as::<Postgres, PhotoRow>("SELECT * FROM photos WHERE id = $1")
                .bind(id)
                .fetch_optional(&**db)
                .await?
                .map(|photo| async {
                    let albums = PhotoAlbum::list(db, &photo.id).await?;
                    let urls = PhotoUrl::list(db, &photo.id).await?;
                    let exif = PhotoExif::list(db, &photo.id).await?;

                    Ok(Self::new(photo, albums, urls, exif))
                }),
        )
        .await
        .map_or(Ok(None), |v| v.map(Some))
    }

    pub async fn get_many<S: AsRef<str> + Sync>(db: &Database, ids: &[S]) -> DbResult<Vec<Photo>> {
        // Create a new query builder
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT * FROM photos WHERE id IN (");

        // Separate the entries by a comma
        let mut separated = query_builder.separated(", ");

        // Add all the id's to the query
        for id in ids.iter() {
            separated.push_bind(id.as_ref());
        }

        // End the 'IN' clause
        separated.push_unseparated(")");

        // Build and execute the query
        join_all(
            query_builder
                .build_query_as::<PhotoRow>()
                .fetch_all(&**db)
                .await?
                .into_iter()
                .map(|photo| async {
                    let albums = PhotoAlbum::list(db, &photo.id).await?;
                    let urls = PhotoUrl::list(db, &photo.id).await?;
                    let exif = PhotoExif::list(db, &photo.id).await?;

                    Ok(Self::new(photo, albums, urls, exif))
                }),
        )
        .await
        .into_iter()
        .collect::<DbResult<Vec<_>>>()
    }

    pub async fn search<S: AsRef<str> + Sync>(
        db: &Database,
        album_id: &Option<S>,
    ) -> DbResult<Vec<Photo>> {
        join_all(
            sqlx::query_as::<Postgres, PhotoRow>(if album_id.is_some() {
                "SELECT \
                    photos.* \
                FROM \
                    photos \
                INNER JOIN \
                    photo_albums \
                ON \
                    photos.id = photo_albums.photo_id \
                WHERE \
                    photo_albums.album_id = $1"
            } else {
                "SELECT * FROM photos"
            })
            .bind(album_id.as_ref().map(|v| v.as_ref()))
            .fetch_all(&**db)
            .await?
            .into_iter()
            .map(|photo| async {
                let albums = PhotoAlbum::list(db, &photo.id).await?;
                let urls = PhotoUrl::list(db, &photo.id).await?;
                let exif = PhotoExif::list(db, &photo.id).await?;

                Ok(Self::new(photo, albums, urls, exif))
            }),
        )
        .await
        .into_iter()
        .collect::<DbResult<Vec<_>>>()
    }

    pub async fn delete(db: &Database, id: &str) -> DbResult<()> {
        sqlx::query("DELETE FROM photos WHERE id = $1")
            .bind(id)
            .execute(&**db)
            .await?;

        Ok(())
    }

    pub async fn delete_many<S: AsRef<str> + Sync>(db: &Database, ids: &[S]) -> DbResult<()> {
        // Create a new query builder
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("DELETE FROM photos WHERE id IN (");

        // Separate the entries by a comma
        let mut separated = query_builder.separated(", ");

        // Add all the id's to the query
        for id in ids.iter() {
            separated.push_bind(id.as_ref());
        }

        // End the 'IN' clause
        separated.push_unseparated(")");

        // Build and execute the query
        query_builder.build().execute(&**db).await?;

        Ok(())
    }
}

impl From<Photo> for proto::Photo {
    fn from(photo: Photo) -> proto::Photo {
        proto::Photo {
            id: Some(photo.id),
            uploaded_at: Some(photo.uploaded_at),
            uploaded_by: None,
            captured_at: photo.captured_at,
            linked: Some(proto::PhotoLinks {
                albums: photo
                    .linked_albums
                    .into_iter()
                    .map(|album| album.into())
                    .collect(),
            }),
            media: Some(proto::PhotoMedia {
                urls: photo.media_urls.into_iter().map(|url| url.into()).collect(),
            }),
            metadata: Some(proto::PhotoMetadata {
                exif: photo
                    .exif_fields
                    .into_iter()
                    .map(|exif| exif.into())
                    .collect(),
            }),
        }
    }
}
