use std::path::PathBuf;
use clap::Parser;

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
}
