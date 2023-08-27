use std::fmt::{Display, Formatter};
use color_eyre::Result;
use proto::{AccessResponse, CreateAlbumRequest, CreateAlbumResponse, CreatePhotoRequest, CreatePhotoResponse};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest_protobuf::{ProtobufRequestExt, ProtobufResponseExt};

pub struct Chroma {
    api_url: String,
    client: Client,
}

pub struct AlbumId(String);
pub struct PhotoId(String);

impl Chroma {
    pub fn new(api_url: String, service_token: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.append(
            "Authorization",
            HeaderValue::from_str(&format!("Service {service_token}"))?
        );

        let client = Client::builder()
            .default_headers(headers)
            .user_agent("Colorizer Pxl-To-Chroma migrator")
            .build()?;
        Ok(Self {
            api_url,
            client,
        })
    }

    fn path(&self, path: &str) -> String {
        format!("{}{}", self.api_url, path)
    }

    pub async fn create_album(&self, name: String) -> Result<AlbumId> {
        let response: CreateAlbumResponse = self.client.post(self.path("/api/v1/album"))
            .protobuf(CreateAlbumRequest {
                name
            })?
            .send()
            .await?
            .error_for_status()?
            .protobuf()
            .await?;

        Ok(AlbumId(response.id))
    }

    pub async fn create_photo(&self, album: &AlbumId, data: Vec<u8>) -> Result<PhotoId> {
        let response: CreatePhotoResponse = self.client.post(self.path("/api/v1/photo"))
            .protobuf(CreatePhotoRequest {
                album_id: album.0.clone(),
                photo_data: data,
            })?
            .send()
            .await?
            .error_for_status()?
            .protobuf()
            .await?;

        Ok(PhotoId(response.photo_id))
    }

    pub async fn access(&self) -> Result<bool> {
        let response: AccessResponse = self.client.post("/access")
            .send()
            .await?
            .error_for_status()?
            .protobuf()
            .await?;

        Ok(response.admin)
    }
}

impl Display for AlbumId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Display for PhotoId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}