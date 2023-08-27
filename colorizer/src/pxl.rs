use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PxlMetadata {
    pub albums: Vec<PxlAlbum>,
}

#[derive(Debug, Deserialize)]
pub struct PxlAlbum {
    pub images: Vec<PxlImage>,
    pub name_display: String,
    pub created: String,
}

#[derive(Debug, Deserialize)]
pub struct PxlImage {
    pub remote_uuid: String,
}