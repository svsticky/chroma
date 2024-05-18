//! Provides the database connection used for Chroma and runs migrations when needed

use std::ops::Deref;

pub use sqlx::error::Error as DatabaseError;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};
use thiserror::Error;

pub use album::*;
pub use photo::*;
pub use user::*;

mod album;
mod photo;
mod user;

const PG_MAX_CONNECTIONS: u32 = 10;

pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Debug, Error)]
pub enum DatabaseInitError {
    #[error("Failed to connect to database: {0}")]
    Connect(#[from] sqlx::Error),
    #[error("Failed to apply migrations: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

/// Postgres connection configuration
pub enum DatabaseConfig<'a> {
    /// Postgres connection configuration using a database connection URL
    Url { url: &'a str },
    /// Postgres connection configuration using separate parameters
    Parameters {
        host: &'a str,
        user: &'a str,
        password: &'a str,
        database: &'a str,
    },
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
    /// Creates a connection to the database and applies migrations if needed
    pub async fn new(config: DatabaseConfig<'_>) -> Result<Database, DatabaseInitError> {
        // Create a connection pool using the provided configuration
        let pool = match config {
            DatabaseConfig::Parameters {
                host,
                user,
                password,
                database,
            } => Self::connect_with_parameters(host, user, password, database).await?,
            DatabaseConfig::Url { url } => Self::connect_with_url(url).await?,
        };

        // Apply the migrations before returning the pool, if needed
        Self::apply_migrations(&pool).await?;

        Ok(Database(pool))
    }

    /// Creates a database connection using a database connection URL
    async fn connect_with_url(url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(PG_MAX_CONNECTIONS)
            .connect(url)
            .await
    }

    /// Creates a database connection using connection parameters
    async fn connect_with_parameters(
        host: &str,
        user: &str,
        password: &str,
        database: &str,
    ) -> Result<Pool<Postgres>, sqlx::Error> {
        let pg_connect = PgConnectOptions::new()
            .host(host)
            .database(database)
            .username(user)
            .password(password);

        PgPoolOptions::new()
            .max_connections(PG_MAX_CONNECTIONS)
            .connect_with(pg_connect)
            .await
    }

    async fn apply_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        let mut connection = pool.acquire().await?;
        sqlx::migrate!().run(&mut connection).await?;

        Ok(())
    }
}
