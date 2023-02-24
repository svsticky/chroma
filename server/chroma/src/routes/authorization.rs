use std::future::Future;
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use actix_web::http::StatusCode;
use crate::koala::get_koala_login_url;
use crate::routes::appdata::WebData;
use thiserror::Error;

pub struct Authorization {
    is_admin: bool,
    name: String,
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

            todo!();

            Ok(Self { // Very much temporary
                name: String::default(),
                is_admin: false,
            })
        })
    }
}

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("Missing or invalid Authorization header")]
    NoHeader(String),
}

impl ResponseError for AuthorizationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NoHeader(_) => StatusCode::SEE_OTHER,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::NoHeader(r) => HttpResponse::build(self.status_code())
                .insert_header(("Location", r.as_str()))
                .finish()
        }
    }
}