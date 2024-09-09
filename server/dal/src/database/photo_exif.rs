use crate::database::{Database, DbResult};
use sqlx::{FromRow, Postgres};

#[derive(Debug, Clone, FromRow)]
pub struct PhotoExif {
    pub photo_id: String,
    pub key: String,
    pub value: String,
}

impl PhotoExif {
    pub async fn create(db: &Database, id: &str, field: &str, value: &str) -> DbResult<PhotoExif> {
        sqlx::query_as::<Postgres, PhotoExif>(
            "INSERT INTO photo_exif \
                    (photo_id, key, value) \
                VALUES \
                    ($1, $2, $3) \
                RETURNING *",
        )
        .bind(id)
        .bind(field)
        .bind(value)
        .fetch_one(&**db)
        .await
    }

    pub async fn get_by_field(
        db: &Database,
        photo_id: &str,
        field: &str,
    ) -> DbResult<Option<PhotoExif>> {
        sqlx::query_as::<Postgres, PhotoExif>(
            "SELECT * FROM photo_exif WHERE photo_id = $1 AND key = $2",
        )
        .bind(photo_id)
        .bind(field)
        .fetch_optional(&**db)
        .await
    }

    pub async fn list(db: &Database, photo_id: &str) -> DbResult<Vec<PhotoExif>> {
        sqlx::query_as::<Postgres, PhotoExif>("SELECT * FROM photo_exif WHERE photo_id = $1")
            .bind(photo_id)
            .fetch_all(&**db)
            .await
    }
}

impl From<PhotoExif> for proto::PhotoExif {
    fn from(photo_exif: PhotoExif) -> proto::PhotoExif {
        proto::PhotoExif {
            key: photo_exif.key,
            value: photo_exif.value,
        }
    }
}
