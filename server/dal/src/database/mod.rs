use refinery::config::{Config, ConfigDbType};
use sqlx::{Pool, Postgres};
use std::ops::Deref;
use thiserror::Error;

mod album;
mod photo;
mod service_token_user;
mod user;

pub use album::*;
pub use photo::*;
pub use service_token_user::*;
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
        passw: Option<&str>,
        database: &str,
    ) -> Result<Database, DatabaseInitError> {
        let mut migration_config = Config::new(ConfigDbType::Postgres)
            .set_db_host(host)
            .set_db_name(database)
            .set_db_user(user);

        let mut pg_connect = PgConnectOptions::new()
            .host(host)
            .database(database)
            .username(user);

        if let Some(passw) = passw {
            migration_config = migration_config.set_db_pass(passw);
            pg_connect = pg_connect.password(passw);
        }

        migrations::migrations::runner()
            .run_async(&mut migration_config)
            .await?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(pg_connect)
            .await?;

        Ok(Database(pool))
    }
}
