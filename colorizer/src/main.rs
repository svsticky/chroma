use crate::args::Args;
use crate::chroma::Chroma;
use crate::pxl::{PxlAlbum, PxlFileTree};
use crate::pxl_metadata::MetadataFile;
use clap::Parser;
use color_eyre::eyre::Error;
use std::path::Path;
use tracing::info;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod args;
mod chroma;
mod pxl;
mod pxl_metadata;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    install_tracing();

    let args = Args::parse();

    info!("Creating Chroma API client");
    let chroma = Chroma::new(args.chroma_api, args.chroma_service_token)?;
    if !chroma.access().await? {
        panic!("Unable to access Chroma API. Is the service token correct?");
    }

    info!("Parsing pxl metadata");
    let file_tree = PxlFileTree::new(args.pxl_dir_base);
    let albums = file_tree.get_albums()?;

    let albums = sort_by_created(&args.metadata_file, albums).await?;

    info!("Processing {} albums.", albums.len());

    // We could parallelize this, but to keep the server load in check and avoid HTTP 429's, we don't.
    for album in albums {
        let images = album.get_photos()?;

        info!("Processing album {}. {} Images.", album.name, images.len());
        let album_id = chroma.create_album(album.name).await?;
        let mut first_photo = None;

        for photo in images {
            let components = photo.s3_url.split("_").collect::<Vec<_>>();
            let first = components.first().ok_or(Error::msg("Invalid src"))?;

            let s3_url = format!("{first}_o.jpg");

            info!("Uploading {}", s3_url);
            let photo_bytes = reqwest::get(s3_url).await?.bytes().await?.to_vec();
            let image_id = chroma.create_photo(&album_id, photo_bytes).await?;

            info!("Created Chroma photo {image_id}");

            if first_photo.is_none() {
                first_photo = Some(image_id);
            }
        }

        match first_photo {
            Some(photo_id) => {
                info!("Updating thumbnail");
                chroma.set_album_thumbnail(&album_id, &photo_id).await?;
            }
            None => {}
        }

        info!("Created Chroma album {album_id}");
    }

    info!("Done");

    Ok(())
}

async fn sort_by_created(
    metadata_file: &Path,
    input: Vec<PxlAlbum>,
) -> color_eyre::Result<Vec<PxlAlbum>> {
    let meta = MetadataFile::open(metadata_file).await?;
    let mut out = Vec::with_capacity(input.len());

    for i in input {
        let created_at = meta
            .albums
            .iter()
            .find(|a| a.name_display.eq(&i.name))
            .map(|e| e.created())
            .unwrap_or(Ok(0))?;

        out.push((i, created_at));
    }

    out.sort_by(|(_, a), (_, b)| a.cmp(b));

    let out = out.into_iter().map(|(a, _)| a).collect::<Vec<_>>();
    Ok(out)
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
