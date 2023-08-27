use crate::koala::{get_koala_login_url, get_token_info};
use crate::routes::appdata::WebData;
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use actix_web::http::StatusCode;
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError};
use dal::database::User;
use std::future::Future;
use std::pin::Pin;
use tap::TapFallible;
use thiserror::Error;
use tracing::{info, trace};

pub struct Authorization {
    pub user_id: i32,
    pub is_admin: bool,
}

impl FromRequest for Authorization {
    type Error = AuthorizationError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let data: &WebData = req.app_data().unwrap();

            let authorization = req
                .headers()
                .get("authorization")
                .ok_or(AuthorizationError::NoHeader(get_koala_login_url(
                    &data.config,
                )))
                .tap_err(|_| trace!("Request is missing authorization header"))?
                .to_str()
                .map_err(|_| AuthorizationError::NoHeader(get_koala_login_url(&data.config)))
                .tap_err(|_| {
                    trace!("Value of authorization header could not be converter to UTF-8 String")
                })?;

            // Check if we're dealing with a service token
            if authorization.starts_with("Service ") {
                let token = authorization.chars().skip(8).collect::<String>();
                if token.is_empty() {
                    return Err(AuthorizationError::InvalidServiceToken);
                }

                return if data.config.service_tokens().contains(&token.as_str()) {
                    Ok(Self {
                        is_admin: true,
                        user_id: 0,
                    })
                } else {
                    Err(AuthorizationError::InvalidServiceToken)
                }
            }

            let user = User::get_by_session_id(&data.db, authorization)
                .await?
                .ok_or(AuthorizationError::InvalidSession(get_koala_login_url(
                    &data.config,
                )))
                .tap_err(|_| trace!("Invalid session ID provided ('{authorization}')"))?;

            // Check if the access token is still valid
            // E.g. it could have been revoked by Koala
            let _token_info = get_token_info(&data.config, &user.access_token)
                .await
                .map_err(|e| match e.status() {
                    Some(v) if v.as_u16() == 401 => {
                        info!("Stored session was valid, tokens for koala were not.");
                        AuthorizationError::InvalidSession(get_koala_login_url(&data.config))
                    }
                    Some(v) if v.as_u16() == 403 => AuthorizationError::Forbidden,
                    _ => AuthorizationError::KoalaUpstream,
                })?;

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
    #[error("Forbidden")]
    Forbidden,
    #[error("Koala has an issue")]
    KoalaUpstream,
    #[error("Provided service token is empty or invalid")]
    InvalidServiceToken,
}

impl ResponseError for AuthorizationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NoHeader(_) => StatusCode::UNAUTHORIZED,
            Self::InvalidSession(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::KoalaUpstream => StatusCode::BAD_GATEWAY,
            Self::InvalidServiceToken => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::NoHeader(r) | Self::InvalidSession(r) => HttpResponse::build(self.status_code())
                .insert_header(("Location", r.as_str()))
                .finish(),
            _ => HttpResponse::build(self.status_code()).body(self.to_string()),
        }
    }
}
