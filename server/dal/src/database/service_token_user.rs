use sqlx::FromRow;
use crate::database::{Database, DbResult};

pub struct ServiceTokenUser<'a> {
    #[allow(unused)]
    db: &'a Database,
    pub id: i32,
    pub service_token: String,
}

#[derive(FromRow)]
struct _ServiceTokenUser {
    id: i32,
    service_token: String,
}

impl _ServiceTokenUser {
    fn to_service_token_user(self, db: &Database) -> ServiceTokenUser {
        ServiceTokenUser {
            db,
            id: self.id,
            service_token: self.service_token,
        }
    }
}

impl<'a> ServiceTokenUser<'a> {
    pub const ID_PREFIX: &'static str = "STU_";
    pub const MAX_ID_LEN: usize = 32;

    pub async fn create<S: AsRef<str>>(db: &'a Database, service_token: S) -> DbResult<ServiceTokenUser<'a>> {
        let id: i32 = sqlx::query_scalar("INSERT INTO service_token_user (service_token) VALUES ($1) RETURNING id")
            .bind(service_token.as_ref())
            .fetch_one(&**db)
            .await?;

        Ok(Self {
            db,
            id,
            service_token: service_token.as_ref().to_string(),
        })
    }

    pub async fn get_by_id<S: AsRef<str>>(db: &'a Database, id: i32) -> DbResult<Option<ServiceTokenUser<'a>>> {
        let user: Option<_ServiceTokenUser> = sqlx::query_as("SELECT * FROM service_token_user WHERE id = $1")
            .bind(id)
            .fetch_optional(&**db)
            .await?;

        Ok(user.map(|user| user.to_service_token_user(db)))
    }

    pub async fn get_by_token<S: AsRef<str>>(db: &'a Database, service_token: S) -> DbResult<Option<ServiceTokenUser<'a>>> {
        let user: Option<_ServiceTokenUser> = sqlx::query_as("SELECT * FROM service_token_user WHERE service_token = $1")
            .bind(service_token.as_ref())
            .fetch_optional(&**db)
            .await?;

        Ok(user.map(|user| user.to_service_token_user(db)))
    }
}