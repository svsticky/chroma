use crate::args::Args;
use crate::chroma::Chroma;
use crate::pxl::{PxlAlbum, PxlFileTree};
use crate::pxl_metadata::MetadataFile;
use clap::Parser;
use color_eyre::eyre::Error;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn};
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

    // Sort them chronologically
    let albums = sort_by_created(&args.metadata_file, albums).await?;

    let albums = if let Some(start_at_dir) = &args.start_at_dir {
        let albums = albums
            .into_iter()
            .skip_while(|a| a.get_dir_name().ne(start_at_dir))
            .collect::<Vec<_>>();

        info!(
            "--start-at-dir provided, starting at directory {start_at_dir}. This leaves {} albums",
            albums.len()
        );

        albums
    } else {
        albums
    };

    info!("Processing {} albums.", albums.len());

    // Quit after this album on SIGQUIT, this is Ctrl + \ on Linux.
    let quit = Arc::new(AtomicBool::new(false));
    tokio::task::spawn({
        let quit = Arc::clone(&quit);

        async move {
            let mut stream = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::quit())
                .expect("Failed to register signal");
            loop {
                stream.recv().await;
                quit.store(true, Ordering::Relaxed);
            }
        }
    });

    // We could parallelize this, but to keep the server load in check and avoid HTTP 429's, we don't.
    for (album_idx, album) in albums.iter().enumerate() {
        if quit.load(Ordering::Relaxed) {
            info!("Ctrl+\\ was pressed, quitting. The name of this album is {}, for resuming next time.", album.name);
            return Ok(());
        }

        let images = album.get_photos()?;

        info!(
            "Processing album {}. {} Images. (Album {}/{})",
            album.name,
            images.len(),
            album_idx + 1,
            albums.len()
        );
        let album_id = chroma.create_album(album.name.clone()).await?;
        let mut first_photo = None;

        let mut sigquit_message_printed = false;
        'photo_loop: for (image_idx, photo) in images.iter().enumerate() {
            if quit.load(Ordering::Relaxed) && !sigquit_message_printed {
                warn!("Ctrl+\\ was pressed, quitting after this album");
                sigquit_message_printed = true;
            }

            let components = photo.s3_url.split("_").collect::<Vec<_>>();
            let first = components.first().ok_or(Error::msg("Invalid src"))?;

            let s3_url = format!("{first}_o.jpg");

            let photo_bytes = loop {
                match reqwest::get(s3_url.clone()).await {
                    Ok(v) => break v.bytes().await?.to_vec(),
                    Err(e) => {
                        warn!("Failed to download image from Pxl S3 bucket: {e}");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                };
            };

            info!(
                "Uploading {} (image {}/{})",
                s3_url,
                image_idx + 1,
                images.len()
            );
            let mut retry_counter = 0;
            let image_id = loop {
                match chroma.create_photo(&album_id, photo_bytes.clone()).await {
                    Ok(v) => break v,
                    Err(e) => {
                        if retry_counter >= 3 {
                            warn!(
                                "Skipping photo '{}' belonging to Chroma album '{}' named '{}'",
                                photo.s3_url, album_id, album.name,
                            );

                            continue 'photo_loop;
                        }

                        warn!("Failed to upload photo to Chroma: {e}. Trying again.");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        retry_counter += 1;
                    }
                }
            };

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
