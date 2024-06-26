extern crate core;

use crate::config::Config;
use crate::routes::appdata::{AlbumIdCache, AppData, SessionIdCache, WebData};
use crate::routes::routable::Routable;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use cabbage::KoalaApi;
use color_eyre::eyre::Error;
use color_eyre::Result;
use dal::database::Database;
use dal::storage_engine::{S3Config, Storage};
use dotenv::dotenv;
use noiseless_tracing_actix_web::NoiselessRootSpanBuilder;
use std::time::Duration;
use tracing::{info, warn};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod config;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

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
    let db = Database::new(config.database_config()?).await?;

    info!("Initializing S3 storage engine");
    let storage = Storage::new(S3Config {
        bucket_name: config.s3_bucket_name.clone().unwrap(),
        endpoint_url: config.s3_endpoint_url.clone().unwrap(),
        region: config.s3_region.clone().unwrap(),
        access_key_id: config.s3_access_key_id.clone().unwrap(),
        secret_access_key: config.s3_secret_access_key.clone().unwrap(),
        use_path_style: config.s3_force_path_style(),
    })
    .await?;

    let appdata = AppData {
        koala: KoalaApi::new(config.koala_base_redirect_uri().clone())?,
        db,
        storage,
        config,
    };

    info!("Starting web server");
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(TracingLogger::<NoiselessRootSpanBuilder>::new())
            .app_data(WebData::new(appdata.clone()))
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
    ))?
    .run()
    .await?;

    Ok(())
}

fn install_tracing() {
    tracing_subscriber::registry()
        .with(layer().compact())
        .with(EnvFilter::from_default_env())
        .init();
}
