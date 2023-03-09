use crate::database::{Database, DbResult};
use rand::Rng;
use sqlx::FromRow;
use time::{Duration, OffsetDateTime};

pub struct User<'a> {
    db: &'a Database,
    pub koala_id: i32,
    pub access_token: String,
    pub refresh_token: String,
    pub oauth_expires_at: i64,
    pub is_admin: bool,
}

pub struct OAuthAccess {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
}

#[derive(FromRow)]
struct _User {
    koala_id: i32,
    access_token: String,
    refresh_token: String,
    expires_at: i64,
    is_admin: bool,
}

impl _User {
    pub fn to_user(self, db: &Database) -> User {
        User {
            db,
            koala_id: self.koala_id,
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            oauth_expires_at: self.expires_at,
            is_admin: self.is_admin,
        }
    }
}

impl<'a> User<'a> {
    pub const SESSION_ID_LEN: usize = 32;
    pub const SESSION_DEFAULT_EXPIRY: Duration = Duration::days(15);

    pub async fn create(
        db: &'a Database,
        koala_id: i32,
        oauth: OAuthAccess,
        admin: bool,
    ) -> DbResult<User<'a>> {
        sqlx::query(
            "INSERT INTO users \
                    (koala_id, access_token, refresh_token, expires_at, is_admin) \
                VALUES \
                    ($1, $2, $3, $4, $5)",
        )
        .bind(koala_id)
        .bind(&oauth.access_token)
        .bind(&oauth.refresh_token)
        .bind(oauth.expires_at)
        .bind(admin)
        .execute(&**db)
        .await?;

        Ok(Self {
            db,
            koala_id,
            access_token: oauth.access_token,
            refresh_token: oauth.refresh_token,
            oauth_expires_at: oauth.expires_at,
            is_admin: admin,
        })
    }

    pub async fn get_by_id(db: &'a Database, koala_id: i32) -> DbResult<Option<User<'a>>> {
        let user: Option<_User> = sqlx::query_as("SELECT koala_id, access_token, refresh_token, expires_at, is_admin FROM users WHERE koala_id = $1")
            .bind(koala_id)
            .fetch_optional(&**db)
            .await?;
        Ok(user.map(|user| user.to_user(db)))
    }

    pub async fn create_session(&self) -> DbResult<String> {
        let session_id: String = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(Self::SESSION_ID_LEN)
            .map(char::from)
            .collect();
        let expires_at = OffsetDateTime::now_utc() + Self::SESSION_DEFAULT_EXPIRY;

        sqlx::query("INSERT INTO user_sessions (id, koala_id, expires_at) VALUES ($1, $2, $3)")
            .bind(&session_id)
            .bind(self.koala_id)
            .bind(expires_at.unix_timestamp())
            .execute(&**self.db)
            .await?;

        Ok(session_id)
    }

    pub async fn get_by_session_id<S: AsRef<str>>(
        db: &'a Database,
        session_id: S,
    ) -> DbResult<Option<User<'a>>> {
        #[derive(FromRow)]
        struct _Session {
            koala_id: i32,
            expires_at: i64,
        }

        let session: Option<_Session> =
            sqlx::query_as("SELECT koala_id, expires_at FROM user_sessions WHERE id = $1")
                .bind(session_id.as_ref())
                .fetch_optional(&**db)
                .await?;

        let session = match session {
            Some(s) => s,
            None => return Ok(None),
        };

        if OffsetDateTime::now_utc().unix_timestamp() >= session.expires_at {
            sqlx::query("DELETE FROM user_sessions WHERE id = $1")
                .bind(session_id.as_ref())
                .execute(&**db)
                .await?;
            return Ok(None);
        }

        User::get_by_id(db, session.koala_id).await
    }

    pub async fn set_tokens(
        &mut self,
        access: String,
        refresh: String,
        expires_at: i64,
    ) -> DbResult<()> {
        sqlx::query("UPDATE users SET access_token = $1, refresh_token = $2, expires_at = $3 WHERE koala_id = $4")
            .bind(&access)
            .bind(&refresh)
            .bind(expires_at)
            .bind(self.koala_id)
            .execute(&**self.db)
            .await?;

        self.access_token = access;
        self.refresh_token = refresh;

        Ok(())
    }
}
