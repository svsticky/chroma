use actix_web::http::StatusCode;
use actix_web::ResponseError;
use thiserror::Error;

pub type WebResult<T> = Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal server error")]
    Database(#[from] dal::database::DatabaseError),
    #[error("The requested resource could not be found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error")]
    StorageEngine(#[from] dal::storage_engine::StorageEngineError),
    #[error("Something went wrong on Koala's end")]
    Koala(reqwest::Error),
    #[error("The requested resource may not be accessed by the authorized user.")]
    Forbidden,
    #[error("Failed to parse timestamp")]
    ChronoParse(#[from] chrono::ParseError),
    #[error("Failed to encode image to WebP")]
    ImageEncoding,
    #[error("Other: {0}")]
    Other(StatusCode),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::StorageEngine(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Koala(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::ChronoParse(_) => StatusCode::BAD_GATEWAY,
            Self::ImageEncoding => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Other(s) => *s,
        }
    }
}
