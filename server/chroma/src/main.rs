extern crate core;

use std::process::ExitCode;

use dotenv::dotenv;
use tracing::level_filters::LevelFilter;
use tracing::{error, warn};
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod app;
mod config;
mod routes;

#[tokio::main]
async fn main() -> ExitCode {
    // Try to load the environment variables from the .env file
    let dotenv_err = dotenv().err();

    // Initialize the tracing logger
    init_tracing();

    // Check if the dot env was loaded correctly, otherwise send a warning
    if let Some(err) = dotenv_err {
        warn!("failed to load .env file: {:#}", err);
    }

    // Run the server
    if let Err(err) = app::run().await {
        error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| error!("because: {}", cause));
        ExitCode::from(1)
    } else {
        ExitCode::from(0)
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
