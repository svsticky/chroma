use crate::database::{Database, DatabaseResult, UserType};
use rand::Rng;
use sqlx::FromRow;
use time::{Duration, OffsetDateTime};

pub struct OAuthAccess {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
}

#[derive(Debug)]
pub struct User<'a> {
    db: &'a Database,
    pub koala_id: i32,
    pub access_token: String,
    pub refresh_token: String,
    pub oauth_expires_at: i64,
    pub is_admin: bool,
    pub name: String,
}

impl<'a> User<'a> {
    pub const SESSION_ID_LEN: usize = 32;
    pub const SESSION_DEFAULT_EXPIRY: Duration = Duration::days(15);

    pub async fn create<S: AsRef<str>>(
        db: &'a Database,
        koala_id: i32,
        oauth: OAuthAccess,
        admin: bool,
        name: S,
    ) -> DatabaseResult<User<'a>> {
        sqlx::query(
            "INSERT INTO users \
                    (koala_id, access_token, refresh_token, expires_at, is_admin, name) \
                VALUES \
                    ($1, $2, $3, $4, $5, $6)",
        )
        .bind(koala_id)
        .bind(&oauth.access_token)
        .bind(&oauth.refresh_token)
        .bind(oauth.expires_at)
        .bind(admin)
        .bind(name.as_ref())
        .execute(&**db)
        .await?;

        Ok(Self {
            db,
            koala_id,
            access_token: oauth.access_token,
            refresh_token: oauth.refresh_token,
            oauth_expires_at: oauth.expires_at,
            is_admin: admin,
            name: name.as_ref().to_string(),
        })
    }

    pub async fn set_is_admin(&mut self, is_admin: bool) -> DatabaseResult<()> {
        if self.is_admin == is_admin {
            return Ok(());
        }

        sqlx::query("UPDATE users SET is_admin = $1 WHERE koala_id = $2")
            .bind(is_admin)
            .bind(&self.koala_id)
            .execute(&**self.db)
            .await?;
        self.is_admin = is_admin;

        Ok(())
    }

    pub async fn get_by_id(db: &'a Database, koala_id: i32) -> DatabaseResult<Option<User<'a>>> {
        let user: Option<UserRow> = sqlx::query_as("SELECT * FROM users WHERE koala_id = $1")
            .bind(koala_id)
            .fetch_optional(&**db)
            .await?;
        Ok(user.map(|user| user.to_user(db)))
    }

    pub async fn create_session(&self) -> DatabaseResult<String> {
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

    pub async fn list(db: &'a Database) -> DatabaseResult<Vec<User<'a>>> {
        let users: Vec<UserRow> = sqlx::query_as(
            "SELECT \
                    * \
                FROM \
                    users",
        )
        .fetch_all(&**db)
        .await?;
        Ok(users.into_iter().map(|f| f.to_user(db)).collect::<Vec<_>>())
    }

    pub async fn get_by_session_id<S: AsRef<str>>(
        db: &'a Database,
        session_id: S,
    ) -> DatabaseResult<Option<User<'a>>> {
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
    ) -> DatabaseResult<()> {
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

    pub async fn get_scopes(&self) -> DatabaseResult<Vec<UserScope>> {
        UserScope::list_for_user(self.db, self.koala_id).await
    }

    pub async fn add_scope<S: AsRef<str>>(
        &self,
        scope: S,
        by: &User<'a>,
    ) -> DatabaseResult<UserScope<'a>> {
        UserScope::add_scope(self.db, self.koala_id, scope, by.koala_id).await
    }

    pub async fn remove_scope(&self, scope: &UserScope<'_>) -> DatabaseResult<()> {
        UserScope::remove_scope(self.db, self.koala_id, &scope.scope).await
    }

    pub async fn remove_scope_by_name<S: AsRef<str>>(&self, scope_name: S) -> DatabaseResult<()> {
        UserScope::remove_scope(self.db, self.koala_id, scope_name.as_ref()).await
    }

    pub fn to_proto(self) -> proto::User {
        proto::User {
            id: self.koala_id,
            name: Some(self.name),
            r#type: UserType::Standard as i32,
        }
    }
}

#[derive(FromRow)]
struct UserRow {
    koala_id: i32,
    access_token: String,
    refresh_token: String,
    expires_at: i64,
    is_admin: bool,
    name: String,
}

impl UserRow {
    pub fn to_user(self, db: &Database) -> User {
        User {
            db,
            koala_id: self.koala_id,
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            oauth_expires_at: self.expires_at,
            is_admin: self.is_admin,
            name: self.name,
        }
    }
}

pub struct UserScope<'a> {
    #[allow(unused)]
    db: &'a Database,
    pub koala_id: i32,
    pub scope: String,
    pub granted_by: i32,
    pub granted_at: i64,
}

impl<'a> UserScope<'a> {
    pub async fn list_for_user(
        db: &'a Database,
        koala_id: i32,
    ) -> DatabaseResult<Vec<UserScope<'a>>> {
        let chroma_scopes: Vec<UserScopeRow> = sqlx::query_as(
            "SELECT koala_id, scope, granted_by, granted_at FROM chroma_scopes WHERE koala_id = $1",
        )
        .bind(koala_id)
        .fetch_all(&**db)
        .await?;

        Ok(chroma_scopes
            .into_iter()
            .map(|f| f.to_user_scope(db))
            .collect::<Vec<_>>())
    }

    async fn remove_scope(db: &Database, koala_id: i32, name: &str) -> DatabaseResult<()> {
        sqlx::query("DELETE FROM chroma_scopes WHERE koala_id = $1 AND scope = $2")
            .bind(koala_id)
            .bind(name)
            .execute(&**db)
            .await?;
        Ok(())
    }

    async fn add_scope<S: AsRef<str>>(
        db: &'a Database,
        to: i32,
        name: S,
        by: i32,
    ) -> DatabaseResult<UserScope> {
        let ts = OffsetDateTime::now_utc().unix_timestamp();

        sqlx::query(
            "INSERT INTO chroma_scopes \
                (koala_id, scope, granted_by, granted_at) \
            VALUES \
                ($1, $2, $3, $4)",
        )
        .bind(to)
        .bind(name.as_ref())
        .bind(by)
        .bind(ts)
        .execute(&**db)
        .await?;

        Ok(UserScope {
            db,
            koala_id: to,
            scope: name.as_ref().to_string(),
            granted_by: by,
            granted_at: ts,
        })
    }
}

#[derive(FromRow)]
struct UserScopeRow {
    koala_id: i32,
    scope: String,
    granted_by: i32,
    granted_at: i64,
}

impl UserScopeRow {
    pub fn to_user_scope(self, db: &Database) -> UserScope {
        UserScope {
            db,
            koala_id: self.koala_id,
            scope: self.scope,
            granted_by: self.granted_by,
            granted_at: self.granted_at,
        }
    }
}
