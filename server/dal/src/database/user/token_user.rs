use crate::database::{Database, DatabaseResult, UserType};
use sqlx::FromRow;

#[derive(Debug)]
pub struct User<'a> {
    #[allow(unused)]
    db: &'a Database,
    pub id: i32,
    pub service_token: String,
}

impl<'a> User<'a> {
    pub async fn create<S: AsRef<str>>(
        db: &'a Database,
        service_token: S,
    ) -> DatabaseResult<User<'a>> {
        let id: i32 = sqlx::query_scalar(
            "INSERT INTO service_token_user (service_token) VALUES ($1) RETURNING id",
        )
        .bind(service_token.as_ref())
        .fetch_one(&**db)
        .await?;

        Ok(Self {
            db,
            id,
            service_token: service_token.as_ref().to_string(),
        })
    }

    pub async fn get_by_id(db: &'a Database, id: i32) -> DatabaseResult<Option<User<'a>>> {
        let user: Option<UserRow> =
            sqlx::query_as("SELECT * FROM service_token_user WHERE id = $1")
                .bind(id)
                .fetch_optional(&**db)
                .await?;

        Ok(user.map(|user| user.to_user(db)))
    }

    pub async fn get_by_token<S: AsRef<str>>(
        db: &'a Database,
        service_token: S,
    ) -> DatabaseResult<Option<User<'a>>> {
        let user: Option<UserRow> =
            sqlx::query_as("SELECT * FROM service_token_user WHERE service_token = $1")
                .bind(service_token.as_ref())
                .fetch_optional(&**db)
                .await?;

        Ok(user.map(|user| user.to_user(db)))
    }

    pub fn to_proto(self) -> proto::User {
        proto::User {
            id: self.id,
            name: None,
            r#type: UserType::Token as i32,
        }
    }
}

#[derive(FromRow)]
struct UserRow {
    id: i32,
    service_token: String,
}

impl UserRow {
    fn to_user(self, db: &Database) -> User {
        User {
            db,
            id: self.id,
            service_token: self.service_token,
        }
    }
}
