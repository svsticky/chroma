use std::ops::Deref;
use refinery::config::{Config, ConfigDbType};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{MySql, Pool};
use thiserror::Error;

mod album;
pub use album::*;

mod photo;
pub use photo::*;

pub use sqlx::error::Error as DatabaseError;

pub type DbResult<T> = Result<T, DatabaseError>;

#[derive(Debug, Clone)]
pub struct Database(Pool<MySql>);

impl Deref for Database {
    type Target = Pool<MySql>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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


pub async fn init_database(host: &str, user: &str, passw: &str, database: &str) -> Result<Database, DatabaseInitError> {
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
    let pool = MySqlPoolOptions::new()
        .connect_with(opts)
        .await?;

    Ok(Database(pool))
}