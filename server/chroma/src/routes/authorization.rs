use std::future::Future;
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use actix_web::http::StatusCode;
use crate::koala::get_koala_login_url;
use crate::routes::appdata::WebData;
use thiserror::Error;
use dal::database::User;

pub struct Authorization {
    pub user_id: u32,
    pub is_admin: bool,
}

impl FromRequest for Authorization {
    type Error = AuthorizationError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let data: &WebData = req.app_data().unwrap();

            let authorization = req.headers()
                .get("authorization")
                .ok_or(AuthorizationError::NoHeader(get_koala_login_url(&data.config)))?
                .to_str()
                .map_err(|_| AuthorizationError::NoHeader(get_koala_login_url(&data.config)))?;

            let user = User::get_by_session_id(&data.db, authorization)
                .await?
                .ok_or(AuthorizationError::InvalidSession(get_koala_login_url(&data.config)))?;

            Ok(Self {
                user_id: user.koala_id,
                is_admin: user.is_admin,
            })
        })
    }
}

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("Internal server error")]
    Database(#[from] dal::database::DatabaseError),
    #[error("Missing or invalid Authorization header")]
    NoHeader(String),
    #[error("Invalid session")]
    InvalidSession(String),
}

impl ResponseError for AuthorizationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NoHeader(_) => StatusCode::UNAUTHORIZED,
            Self::InvalidSession(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::Database(_) => HttpResponse::build(self.status_code())
                .body(self.to_string()),
            Self::NoHeader(r) | Self::InvalidSession(r) => HttpResponse::build(self.status_code())
                .insert_header(("Location", r.as_str()))
                .finish()
        }
    }
}