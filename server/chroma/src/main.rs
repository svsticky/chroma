extern crate core;

use dotenv::dotenv;
use tracing::level_filters::LevelFilter;
use tracing::{error, warn};
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod config;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    // Try to load the environment variables from the .env file
    let dotenv_err = dotenv().err();

    // Initialize the tracing logger
    init_tracing();

    // Check if the dot env was loaded correctly, otherwise send a warning
    if let Some(err) = dotenv_err {
        warn!("failed to load .env file: {:#}", err);
    }

    // Run the server
    if let Err(err) = server::run().await {
        error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| error!("because: {}", cause));
        std::process::exit(1);
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
