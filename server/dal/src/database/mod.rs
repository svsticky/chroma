use refinery::config::{Config, ConfigDbType};
use sqlx::{Pool, Postgres};
use std::ops::Deref;
use thiserror::Error;

mod album;
mod photo;
mod user;

pub use album::*;
pub use photo::*;
pub use sqlx::error::Error as DatabaseError;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
pub use user::*;

pub type DbResult<T> = Result<T, DatabaseError>;

mod migrations {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[derive(Debug, Error)]
pub enum DatabaseInitError {
    #[error("Failed to apply migrations")]
    Refinery(#[from] refinery::Error),
    #[error("Failed to create connection pool")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug, Clone)]
pub struct Database(Pool<Postgres>);

impl Deref for Database {
    type Target = Pool<Postgres>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Database {
    pub async fn new(
        host: &str,
        user: &str,
        passw: &str,
        database: &str,
    ) -> Result<Database, DatabaseInitError> {
        let mut cfg = Config::new(ConfigDbType::Postgres)
            .set_db_host(host)
            .set_db_name(database)
            .set_db_user(user)
            .set_db_pass(passw);
        migrations::migrations::runner().run_async(&mut cfg).await?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(
                PgConnectOptions::new()
                    .host(host)
                    .database(database)
                    .username(user)
                    .password(passw),
            )
            .await?;

        Ok(Database(pool))
    }
}
