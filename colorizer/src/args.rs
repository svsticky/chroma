use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub pxl_dir_base: PathBuf,

    /// The base URL for the Chroma API.
    /// E.g., `http://localhost:8080`
    #[clap(long)]
    pub chroma_api: String,
    /// The service token for the Chroma API.
    /// Must be provided via an environmental variable.
    #[clap(env)]
    pub chroma_service_token: String,

    /// Name of the Pxl album directory to start at, all albums before (chronologically) are ignored.
    #[clap(long)]
    pub start_at_dir: Option<String>,

    /// Pxl `state.json` file.
    #[clap(long)]
    pub metadata_file: PathBuf,
}
