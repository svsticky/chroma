use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The name for the Pxl metadata file on S3.
    #[clap(long)]
    pub pxl_metadata_file_name: String,

    /// The base URL for the Chroma API.
    /// E.g., `http://localhost:8080`
    #[clap(long)]
    pub chroma_api: String,
    /// The service token for the Chroma API.
    #[clap(long)]
    pub chroma_service_token: String,

    /// The name of the S3 bucket that should be used
    #[clap(long)]
    pub s3_bucket_name: String,
    /// The S3 region the endpoint is located in
    #[clap(long)]
    pub s3_region: String,
    /// S3 endpoint URL
    #[clap(long)]
    pub s3_endpoint_url: String,
    /// S3 secret key ID
    #[clap(long)]
    pub s3_access_key_id: String,
    /// S3 secret access key
    #[clap(long)]
    pub s3_secret_access_key: String,
    /// Force the use of path style bucket addressing.
    /// This should be `true` if the S3 endpoint is MinIO,
    /// but should be `false` or left unspecified when targeting
    /// Amazon S3.
    #[clap(long)]
    pub s3_force_path_style: Option<bool>,
}
