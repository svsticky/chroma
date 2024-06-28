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
    StorageEngine(#[from] dal::storage_engine::error::StorageError),
    #[error("Something went wrong on Koala's end")]
    Koala(reqwest::Error),
    #[error("The requested resource may not be accessed by the authorized user.")]
    Forbidden,
    #[error("Failed to parse timestamp")]
    ChronoParse(#[from] chrono::ParseError),
    #[error("Other: {0}")]
    Other(StatusCode),
    #[error("{0}")]
    ImagePipeline(#[from] ImagePipelineError),
    #[error("{0}")]
    ImageEncoding(#[from] image::ImageError),
    #[error("Failed to decode WebP image")]
    WebpDecode,
    #[error("Slow down. Too many requests")]
    Ratelimit,
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
            Self::ImagePipeline(e) => e.status_code(),
            Self::ImageEncoding(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::WebpDecode => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Other(s) => *s,
            Self::Ratelimit => StatusCode::TOO_MANY_REQUESTS,
        }
    }
}

#[derive(Debug, Error)]
pub enum ImagePipelineError {
    #[error("{0}")]
    StringFromUtf8(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    DateTimeParse(#[from] chrono::format::ParseError),
    #[error("EXIF metadata field '{0}' is of an invalid datatype")]
    InvalidExifFieldType(&'static str),
    #[error("Field '{0}' missing from EXIF metadata")]
    MissingExifField(&'static str),
    #[error("{0}")]
    ExifParsing(#[from] exif::Error),
    #[error("{0}")]
    WebpEncoding(String),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    ImgPartsDecode(#[from] img_parts::Error),
}

impl ImagePipelineError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::StringFromUtf8(_) => StatusCode::BAD_REQUEST,
            Self::DateTimeParse(_) => StatusCode::BAD_REQUEST,
            Self::InvalidExifFieldType(_) => StatusCode::BAD_REQUEST,
            Self::MissingExifField(_) => StatusCode::BAD_REQUEST,
            Self::ExifParsing(_) => StatusCode::BAD_REQUEST,
            Self::WebpEncoding(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ImgPartsDecode(_) => StatusCode::BAD_REQUEST,
        }
    }
}
