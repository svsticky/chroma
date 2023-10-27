use sqlx::{Pool, Postgres};
use std::ops::Deref;

mod album;
mod photo;
mod service_token_user;
mod user;

pub use album::*;
pub use photo::*;
pub use service_token_user::*;
pub use sqlx::error::Error as DatabaseError;
use sqlx::migrate::Migrator;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::sqlx_macros::migrate;
use thiserror::Error;
pub use user::*;

pub type DbResult<T> = Result<T, DatabaseError>;

#[derive(Debug, Error)]
pub enum DatabaseInitError {
    #[error("Failed to connect to database: {0}")]
    Connect(#[from] sqlx::Error),
    #[error("Failed to apply migrations: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

const MIGRATOR: Migrator = migrate!("./migrations");
const PG_MAX_CONNECTIONS: u32 = 10;

#[derive(Debug, Clone)]
pub struct Database(Pool<Postgres>);

impl Deref for Database {
    type Target = Pool<Postgres>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub enum DbConfig<'a> {
    Url {
        url: &'a str,
    },
    Parameters {
        host: &'a str,
        user: &'a str,
        passw: &'a str,
        database: &'a str,
        port: u16
    },
}

impl Database {
    pub async fn new(config: DbConfig<'_>) -> Result<Database, DatabaseInitError> {
        let pool = match config {
            DbConfig::Parameters {
                host,
                user,
                passw,
                database,
                port
            } => Self::configure_with_parameters(host, user, passw, database, port).await?,
            DbConfig::Url { url } => Self::configure_with_url(url).await?,
        };

        Self::apply_migrations(&pool).await?;

        Ok(Database(pool))
    }

    async fn apply_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        let mut connection = pool.acquire().await?;
        MIGRATOR.run(&mut connection).await?;

        Ok(())
    }

    async fn configure_with_url(url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(PG_MAX_CONNECTIONS)
            .connect(url)
            .await
    }

    async fn configure_with_parameters(
        host: &str,
        user: &str,
        passw: &str,
        database: &str,
        port: u16
    ) -> Result<Pool<Postgres>, sqlx::Error> {
        let pg_connect = PgConnectOptions::new()
            .host(host)
            .port(port)
            .database(database)
            .username(user)
            .password(passw);

        PgPoolOptions::new()
            .max_connections(PG_MAX_CONNECTIONS)
            .connect_with(pg_connect)
            .await
    }
}
