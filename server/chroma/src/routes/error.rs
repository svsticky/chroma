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
    S3(#[from] dal::s3::S3Error),
    #[error("Something went wrong on Koala's end")]
    Koala(reqwest::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::S3(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Koala(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}