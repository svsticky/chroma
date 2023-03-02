use refinery::config::{Config, ConfigDbType};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{MySql, Pool};
use std::ops::Deref;
use thiserror::Error;

mod album;
mod photo;
mod user;

pub use album::*;
pub use photo::*;
pub use sqlx::error::Error as DatabaseError;
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
pub struct Database(Pool<MySql>);

impl Deref for Database {
    type Target = Pool<MySql>;
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
        let mut cfg = Config::new(ConfigDbType::Mysql)
            .set_db_host(host)
            .set_db_name(database)
            .set_db_user(user)
            .set_db_pass(passw);
        migrations::migrations::runner().run(&mut cfg)?;

        let opts = MySqlConnectOptions::new()
            .host(host)
            .database(database)
            .username(user)
            .password(passw);
        let pool = MySqlPoolOptions::new().connect_with(opts).await?;

        Ok(Database(pool))
    }
}
