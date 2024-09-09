use crate::database::{Database, DbResult};
use sqlx::{FromRow, Postgres};

#[derive(Debug, Clone, FromRow)]
pub struct PhotoAlbum {
    pub photo_id: String,
    pub album_id: String,
}

impl PhotoAlbum {
    pub async fn create(db: &Database, photo_id: &str, album_id: &str) -> DbResult<PhotoAlbum> {
        sqlx::query_as::<Postgres, PhotoAlbum>(
            "INSERT INTO photo_albums \
                    (photo_id, album_id) \
                VALUES \
                    ($1, $2) \
                RETURNING *",
        )
        .bind(photo_id)
        .bind(album_id)
        .fetch_one(&**db)
        .await
    }

    pub async fn list(db: &Database, photo_id: &str) -> DbResult<Vec<PhotoAlbum>> {
        sqlx::query_as::<Postgres, PhotoAlbum>("SELECT * FROM photo_albums WHERE photo_id = $1")
            .bind(photo_id)
            .fetch_all(&**db)
            .await
    }
}

impl From<PhotoAlbum> for proto::PhotoAlbum {
    fn from(photo_album: PhotoAlbum) -> proto::PhotoAlbum {
        proto::PhotoAlbum {
            id: photo_album.album_id,
        }
    }
}
