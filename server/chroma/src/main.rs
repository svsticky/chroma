extern crate core;

use crate::config::Config;
use crate::routes::appdata::{AppData, WebData};
use crate::routes::routable::Routable;
use actix_cors::Cors;
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_web::http::Method;
use actix_web::{App, HttpServer};
use color_eyre::eyre::Error;
use color_eyre::Result;
use dal::database::Database;
use dal::s3::S3Config;
use dal::storage_engine::StorageEngine;
use noiseless_tracing_actix_web::NoiselessRootSpanBuilder;
use std::path::PathBuf;
use tracing::{info, warn};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod config;
mod koala;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    install_tracing();

    info!("Starting");

    info!("Parsing config");
    let config = Config::parse()?;

    if !config.validate() {
        return Err(Error::msg("Config is not valid."));
    }

    if !config.service_tokens.is_empty() {
        warn!("There are service tokens configured, Make sure these are, and stay, confidential!");
    }

    info!("Initializing database");
    let db = Database::new(
        &config.db_host,
        &config.db_username,
        config.db_password.as_deref(),
        &config.db_database,
    )
    .await?;

    info!("Initializing storage engine");
    let storage = match config.storage_engine {
        config::StorageEngine::S3 => {
            StorageEngine::new_s3(S3Config {
                bucket_name: config.s3_bucket_name.clone().unwrap(),
                endpoint_url: config.s3_endpoint_url.clone().unwrap(),
                region: config.s3_region.clone().unwrap(),
                access_key_id: config.s3_access_key_id.clone().unwrap(),
                secret_access_key: config.s3_secret_access_key.clone().unwrap(),
                use_path_style: config.s3_force_path_style(),
            })
            .await?
        }
        config::StorageEngine::File => {
            StorageEngine::new_file(PathBuf::from(config.file_base.clone().unwrap())).await?
        }
    };

    let appdata = AppData {
        db,
        storage,
        config,
    };

    info!("Starting web server");
    let governor_config = configure_governor()?;
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(TracingLogger::<NoiselessRootSpanBuilder>::new())
            .wrap(Governor::new(&governor_config))
            .app_data(WebData::new(appdata.clone()))
            .configure(routes::Router::configure)
    })
    .bind(&format!(
        "0.0.0.0:{}",
        std::env::var("HTTP_PORT").unwrap_or("8000".into())
    ))?
    .run()
    .await?;

    Ok(())
}

fn configure_governor() -> Result<GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware>> {
    GovernorConfigBuilder::default()
        .per_second(10)
        .methods(vec![
            Method::DELETE,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::GET,
        ])
        .burst_size(100)
        .finish()
        .ok_or(Error::msg("Governor config is invalid."))
}

fn install_tracing() {
    tracing_subscriber::registry()
        .with(layer().compact())
        .with(EnvFilter::from_default_env())
        .init();
}
