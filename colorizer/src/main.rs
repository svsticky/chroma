use crate::args::Args;
use crate::chroma::Chroma;
use crate::pxl::PxlMetadata;
use crate::s3::{S3Config, S3};
use clap::Parser;
use tracing::info;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

mod args;
mod chroma;
mod pxl;
mod s3;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    install_tracing();

    let args = Args::parse();

    info!("Creating S3 client");
    let s3 = S3::new(S3Config {
        bucket_name: args.s3_bucket_name,
        endpoint_url: args.s3_endpoint_url,
        region: args.s3_region,
        access_key_id: args.s3_access_key_id,
        secret_access_key: args.s3_secret_access_key,
        use_path_style: args.s3_force_path_style.unwrap_or(false),
    })?;

    info!("Creating Chroma API client");
    let chroma = Chroma::new(args.chroma_api, args.chroma_service_token)?;
    if !chroma.access().await? {
        panic!("Unable to access Chroma API. Is the service token correct?");
    }

    info!("Opening Pxl metadata file");
    let metadata = s3.get_file(&args.pxl_metadata_file_name).await?;
    let metadata: PxlMetadata = serde_json::from_slice(&metadata)?;

    info!("Processing {} albums.", metadata.albums.len());

    // We could parallelize this, but to keep the server load in check and avoid HTTP 429's, we don't.
    for album in metadata.albums {
        info!(
            "Processing album {}. {} Images.",
            album.name_display,
            album.images.len()
        );
        let album_id = chroma.create_album(album.name_display).await?;

        for photo in album.images {
            info!("Processing {}", photo.remote_uuid);

            let uuid = Uuid::parse_str(&photo.remote_uuid)?;
            let s3_key = format!("{}_o.jpg", uuid.to_string());

            let photo_bytes = s3.get_file(s3_key).await?;
            let image_id = chroma.create_photo(&album_id, photo_bytes).await?;

            info!("Creating Chroma photo {image_id}");
        }

        info!("Created Chroma album {album_id}");
    }

    info!("Done");

    Ok(())
}

fn install_tracing() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "WARN,colorizer=INFO");
    }

    tracing_subscriber::registry()
        .with(layer().compact())
        .with(EnvFilter::from_default_env())
        .init();
}
