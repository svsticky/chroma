use actix_cors::Cors;
use actix_web::{App, HttpServer};
use color_eyre::Result;
use noiseless_tracing_actix_web::NoiselessRootSpanBuilder;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::config::Config;
use crate::routes::appdata::{AppData, WebData};
use crate::routes::routable::Routable;

mod routes;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    install_tracing();

    info!("Starting");

    info!("Parsing config");
    let config = Config::parse()?;

    info!("Initializing database");
    let db = dal::database::init_database(
        &config.db_host,
        &config.db_username,
        &config.db_password,
        &config.db_database
    ).await?;

    info!("Initializing S3 connection");
    let s3 = dal::s3::init_s3(
        config.s3_app_name.clone(),
        &config.s3_endpoint_url,
        config.s3_region.clone(),
        &config.s3_access_key_id,
        &config.s3_secret_access_key
    ).await?;

    let appdata = AppData {
        db,
        s3,
        config
    };

    info!("Starting web server");
    HttpServer::new(move || App::new()
        .wrap(Cors::permissive())
        .wrap(TracingLogger::<NoiselessRootSpanBuilder>::new())
        .app_data(WebData::new(appdata.clone()))
        .configure(routes::Router::configure)
    );

    Ok(())
}

fn install_tracing() {
    tracing_subscriber::registry()
        .with(layer().compact())
        .with(EnvFilter::from_default_env())
        .init();
}