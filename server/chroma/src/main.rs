extern crate core;

use std::time::Duration;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use anyhow::{anyhow, bail, Result};
use cabbage::KoalaApi;
use dotenv::dotenv;
use noiseless_tracing_actix_web::NoiselessRootSpanBuilder;
use tracing::level_filters::LevelFilter;
use tracing::{info, warn};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use dal::database::Database;
use dal::storage_engine::{S3Config, Storage};

use crate::config::Config;
use crate::exit::Exit;
use crate::routes::appdata::{AlbumIdCache, AppData, Ratelimits, SessionIdCache, WebData};
use crate::routes::routable::Routable;

mod config;
mod exit;
mod routes;

/// Run the chroma server and will block until the server is stopped or crashes
///
/// # Errors
///
/// - If the configuration is invalid
/// - If the database connection cannot be completed
/// - If the S3 configuration is invalid
/// - If the port for the server is already in use
/// - If a problem occurs in one of the server routes
#[tokio::main]
async fn main() -> Exit {
    // Try to load the environment variables from the .env file
    let dotenv_err = dotenv().err();

    // Initialize the tracing logger
    init_tracing();

    // Check if the dot env was loaded correctly, otherwise send a warning
    if let Some(err) = dotenv_err {
        warn!("failed to load .env file: {:#}", err);
    }

    // Initialize chroma's core components
    let config = match init_config() {
        Ok(v) => v,
        Err(err) => return Exit::Err(err),
    };
    let db = match init_database(&config).await {
        Ok(v) => v,
        Err(err) => return Exit::Err(err),
    };
    let storage = match init_storage(&config).await {
        Ok(v) => v,
        Err(err) => return Exit::Err(err),
    };
    let koala = match KoalaApi::new(config.koala_base_redirect_uri().clone()) {
        Ok(v) => v,
        Err(err) => return Exit::Err(err.into()),
    };

    // Package the core components up into the AppData struct
    let app_data = AppData {
        koala,
        db,
        storage,
        config,
        ratelimits: Ratelimits::new(),
    };

    // Run the webserver using the AppData until stopped or crash
    match start_webserver(app_data).await {
        Ok(_) => Exit::Ok,
        Err(err) => Exit::Err(err),
    }
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(layer().compact())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
}

/// Parses the configuration using
fn init_config() -> anyhow::Result<Config> {
    info!("parsing config");
    let config = Config::parse().map_err(|err| anyhow!("failed to parse config: {:#}", err))?;

    if !config.validate() {
        bail!("config is not valid");
    }

    if !config.service_tokens.is_empty() {
        warn!("there are service tokens configured; make sure these are, and stay, confidential!");
    }

    Ok(config)
}

async fn init_database(config: &Config) -> anyhow::Result<Database> {
    info!("initializing database connection");
    Database::new(config.database_config().unwrap())
        .await
        .map_err(|err| anyhow!("failed to initialize database connection: {:#}", err))
}

async fn init_storage(config: &Config) -> anyhow::Result<Storage> {
    info!("initializing S3 storage engine");
    Storage::new(S3Config {
        bucket_name: config.s3_bucket_name.clone().unwrap(),
        endpoint_url: config.s3_endpoint_url.clone().unwrap(),
        region: config.s3_region.clone().unwrap(),
        access_key_id: config.s3_access_key_id.clone().unwrap(),
        secret_access_key: config.s3_secret_access_key.clone().unwrap(),
        use_path_style: config.s3_force_path_style(),
        create_bucket: config.s3_create_bucket_on_startup(),
    })
    .await
    .map_err(|err| anyhow!("failed to initialize S3 storage engine: {:#}", err))
}

async fn start_webserver(app_data: AppData) -> Result<()> {
    info!("starting web server");
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(TracingLogger::<NoiselessRootSpanBuilder>::new())
            .app_data(WebData::new(app_data.clone()))
            .app_data(web::Data::new(
                SessionIdCache::builder()
                    .max_capacity(10000)
                    .time_to_live(Duration::from_secs(30))
                    .build(),
            ))
            .app_data(web::Data::new(
                AlbumIdCache::builder().max_capacity(10000).build(),
            ))
            .configure(routes::Router::configure)
    })
    .bind(&format!(
        "0.0.0.0:{}",
        std::env::var("HTTP_PORT").unwrap_or("8000".into())
    ))
    .map_err(|err| anyhow!("failed to bind web server to port 8000: {:#}", err))?
    .run()
    .await?;

    Ok(())
}
