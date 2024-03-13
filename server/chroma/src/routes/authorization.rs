use crate::routes::appdata::{SessionIdCache, WebData};
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use actix_web::http::StatusCode;
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError};
use dal::database::{ChromaScope, Database, DbResult, ServiceTokenUser, User, UserType};
use std::future::Future;
use std::pin::Pin;
use tap::TapFallible;
use thiserror::Error;
use tracing::{info, trace};

#[derive(Clone)]
pub struct Authorization {
    pub user: AuthorizedUser,
    pub is_admin: bool,
}

#[derive(Clone)]
pub enum AuthorizedUser {
    Koala { koala_id: i32 },
    Service { token: String },
}

impl Authorization {
    pub async fn to_dal_user_type(&self, db: &Database) -> DbResult<UserType> {
        let user_type = match &self.user {
            AuthorizedUser::Koala { koala_id } => UserType::Koala(*koala_id),
            AuthorizedUser::Service { token } => {
                match ServiceTokenUser::get_by_token(db, token).await? {
                    Some(stu) => UserType::ServiceToken(stu.id),
                    None => {
                        let stu = ServiceTokenUser::create(db, token).await?;
                        UserType::ServiceToken(stu.id)
                    }
                }
            }
        };

        Ok(user_type)
    }

    pub async fn list_scopes(&self, db: &Database) -> DbResult<String> {
        Ok(match self.user {
            AuthorizedUser::Koala { koala_id } => ChromaScope::list_for_user(db, koala_id)
                .await?
                .into_iter()
                .map(|scope| scope.scope)
                .collect::<Vec<_>>()
                .join(" ")
                .to_string(),
            AuthorizedUser::Service { .. } => String::new(),
        })
    }

    pub async fn has_scope<S: AsRef<str>>(&self, db: &Database, scope: S) -> DbResult<bool> {
        Ok(match self.user {
            AuthorizedUser::Koala { koala_id } => ChromaScope::list_for_user(db, koala_id)
                .await?
                .into_iter()
                .any(|f| f.scope.eq(scope.as_ref())),
            AuthorizedUser::Service { .. } => true,
        })
    }
}

impl FromRequest for Authorization {
    type Error = AuthorizationError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let data: &WebData = req.app_data().unwrap();
            let oauth = data.koala.oauth_api(data.config.oauth_client_config());

            let authorization_id = req
                .headers()
                .get("authorization")
                .ok_or(AuthorizationError::NoHeader(oauth.get_login_redirect_uri()))
                .tap_err(|_| trace!("Request is missing authorization header"))?
                .to_str()
                .map_err(|_| AuthorizationError::NoHeader(oauth.get_login_redirect_uri()))
                .tap_err(|_| {
                    trace!("Value of authorization header could not be converter to UTF-8 String")
                })?;

            // Check the cache
            let session_cache: &SessionIdCache = req.app_data().unwrap();
            match session_cache.get(authorization_id).await {
                Some(v) => return Ok(v),
                None => {}
            }

            // Check if we're dealing with a service token
            if authorization_id.starts_with("Service ") {
                let token = authorization_id.chars().skip(8).collect::<String>();
                if token.is_empty() {
                    return Err(AuthorizationError::InvalidServiceToken);
                }

                return if data.config.service_tokens().contains(&token.as_str()) {
                    Ok(Self {
                        is_admin: true,
                        user: AuthorizedUser::Service { token },
                    })
                } else {
                    Err(AuthorizationError::InvalidServiceToken)
                };
            }

            let mut user = User::get_by_session_id(&data.db, authorization_id)
                .await?
                .ok_or(AuthorizationError::InvalidSession(
                    oauth.get_login_redirect_uri(),
                ))
                .tap_err(|_| trace!("Invalid session ID provided ('{authorization_id}')"))?;

            // Check if the access token is still valid
            // E.g. it could have been revoked by Koala
            let user_info =
                oauth
                    .get_userinfo(&user.access_token)
                    .await
                    .map_err(|e| match e.status() {
                        Some(v) if v.as_u16() == 401 => {
                            info!("Stored session was valid, tokens for koala were not.");
                            AuthorizationError::InvalidSession(oauth.get_login_redirect_uri())
                        }
                        Some(v) if v.as_u16() == 403 => AuthorizationError::Forbidden,
                        _ => AuthorizationError::KoalaUpstream,
                    })?;

            user.set_is_admin(user_info.is_admin).await?;

            let authorization = Self {
                user: AuthorizedUser::Koala {
                    koala_id: user.koala_id,
                },
                is_admin: user.is_admin,
            };

            session_cache
                .insert(authorization_id.to_string(), authorization.clone())
                .await;

            Ok(authorization)
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
