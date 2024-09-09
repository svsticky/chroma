use sqlx::{FromRow, Postgres};

use crate::database::{Database, DbResult};

#[derive(Debug, Clone, FromRow)]
pub struct PhotoUrl {
    pub photo_id: String,
    pub url: String,
    pub size: String,
    pub width: i32,
    pub height: i32,
}

impl PhotoUrl {
    pub async fn create<S: AsRef<str>>(
        db: &Database,
        photo_id: &str,
        url: &str,
        size: S,
        width: i32,
        height: i32,
    ) -> DbResult<PhotoUrl> {
        sqlx::query_as::<Postgres, PhotoUrl>(
            "INSERT INTO photo_urls \
                (photo_id, url, size, width, height) \
            VALUES \
                ($1, $2, $3, $4, $5) \
            RETURNING *",
        )
        .bind(photo_id)
        .bind(url)
        .bind(size.as_ref())
        .bind(width)
        .bind(height)
        .fetch_one(&**db)
        .await
    }

    pub async fn get_by_size<S: AsRef<str>>(
        db: &Database,
        photo_id: &str,
        size: S,
    ) -> DbResult<Option<PhotoUrl>> {
        sqlx::query_as::<Postgres, PhotoUrl>(
            "SELECT * FROM photo_urls WHERE photo_id = $1 AND size = $2",
        )
        .bind(photo_id)
        .bind(size.as_ref())
        .fetch_optional(&**db)
        .await
    }

    pub async fn list(db: &Database, photo_id: &str) -> DbResult<Vec<PhotoUrl>> {
        sqlx::query_as::<Postgres, PhotoUrl>("SELECT * FROM photo_urls WHERE photo_id = $1")
            .bind(photo_id)
            .fetch_all(&**db)
            .await
    }
}

impl From<PhotoUrl> for proto::PhotoUrl {
    fn from(photo_url: PhotoUrl) -> proto::PhotoUrl {
        proto::PhotoUrl {
            url: photo_url.url,
            size: photo_url.size,
            dimensions: Some(proto::PhotoDimensions {
                width: photo_url.width as i64,
                height: photo_url.height as i64,
            }),
        }
    }
}
