extern crate core;

<<<<<<< Updated upstream
use crate::config::Config;
use crate::routes::appdata::{AlbumIdCache, AppData, SessionIdCache, WebData};
use crate::routes::routable::Routable;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use cabbage::KoalaApi;
use color_eyre::eyre::Error;
use color_eyre::Result;
use dal::database::Database;
use dal::s3::S3Config;
use dal::storage_engine::StorageEngine;
use noiseless_tracing_actix_web::NoiselessRootSpanBuilder;
use std::path::PathBuf;
use std::time::Duration;
use tracing::{info, warn};
use tracing_actix_web::TracingLogger;
=======
use std::{env, process};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::io::Result;
use tracing::{error, info};
>>>>>>> Stashed changes
use tracing_subscriber::fmt::layer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment values from .env file
    dotenv().ok();

    // Add the tracing logger
    tracing_subscriber::registry()
        .with(layer().compact())
        .with(EnvFilter::from_default_env())
        .init();

    // Connect to the database
    info!("initializing database connection");
    if let Err(e) = init_database_connection().await {
        error!("Failed to initialize the database connection: {e}");
        process::exit(1);
    }

    // Create the storage engine based on the environment variables
    info!("initializing storage engine");
    if let Err(e) = init_storage_engine().await {
        error!("Failed to initialize the storage engine: {e}");
        process::exit(1);
    }

    // Start the webserver
    info!("starting webserver");
    if let Err(e) = start_server().await {
        error!("Failed to start server: {e}");
        process::exit(1);
    }

    // Webserver has been stopped successfully
    info!("webserver stopped; exiting chroma");

    Ok(())
}

<<<<<<< Updated upstream
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
=======
async fn init_database_connection() -> Result<()> {
    // info!("Initializing database");
    // let db = Database::new(config.database_config()?).await?;
>>>>>>> Stashed changes

    Ok(())
}

async fn init_storage_engine() -> Result<()> {
    // let storage = match config.storage_engine {
    //     config::StorageEngine::S3 => {
    //         StorageEngine::new_s3(S3Config {
    //             bucket_name: config.s3_bucket_name.clone().unwrap(),
    //             endpoint_url: config.s3_endpoint_url.clone().unwrap(),
    //             region: config.s3_region.clone().unwrap(),
    //             access_key_id: config.s3_access_key_id.clone().unwrap(),
    //             secret_access_key: config.s3_secret_access_key.clone().unwrap(),
    //             use_path_style: config.s3_force_path_style(),
    //         })
    //             .await?
    //     }
    //     config::StorageEngine::File => {
    //         StorageEngine::new_file(PathBuf::from(config.file_base.clone().unwrap())).await?
    //     }
    // };
    Ok(())
}

async fn start_server() -> Result<()> {
    HttpServer::new(move || {
        App::new()
    })
        .bind(&format!(
            "0.0.0.0:{}",
            env::var("HTTP_PORT").unwrap_or("8000".into())
        ))?
        .run()
        .await
}