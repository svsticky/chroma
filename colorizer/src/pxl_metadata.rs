use serde::Deserialize;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncReadExt;

#[derive(Deserialize)]
pub struct MetadataFile {
    pub albums: Vec<AlbumMetadata>,
}

#[derive(Deserialize)]
pub struct AlbumMetadata {
    created: String,
    pub name_display: String,
}

impl MetadataFile {
    pub async fn open(path: &Path) -> color_eyre::Result<Self> {
        let mut f = fs::File::open(path).await?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await?;

        Ok(serde_json::from_slice(&buf)?)
    }
}

impl AlbumMetadata {
    pub fn created(&self) -> color_eyre::Result<i64> {
        let dt = chrono::DateTime::parse_from_rfc3339(&self.created)?;
        Ok(dt.timestamp())
    }
}
