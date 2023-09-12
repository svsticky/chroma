use crate::config::Config;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::trace;

/// Get the URL to redirect a client to when they should log in with Koala
pub fn get_koala_login_url(config: &Config) -> String {
    format!(
        "{}/api/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}",
        config.koala_base_redirect_uri(),
        config.koala_client_id,
        config.koala_oauth_redirect_uri,
        "member-read+openid+email+profile"
    )
}

#[derive(Debug, Deserialize)]
pub struct ExchangeResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub created_at: String,
    pub email: String,
    pub credentials_type: CredentialsType,
    /// Koala ID
    pub credentials_id: i32,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum CredentialsType {
    Admin,
    Member,
}

#[derive(Debug, Serialize)]
struct ExchangeRequest<'a> {
    grant_type: GrantType,
    code: Option<&'a str>,
    client_id: &'a str,
    client_secret: &'a str,
    redirect_uri: &'a str,
    refresh_token: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum GrantType {
    /// OAuth authorization_code flow
    AuthorizationCode,
    /// OAuth refresh_token flow
    #[allow(unused)]
    RefreshToken,
}

/// Get the URl to use to exchange an access code for an OAuth token pair,
/// or to refresh an existing access token for a new one.
fn get_koala_token_url(config: &Config) -> String {
    format!("{}/api/oauth/token", config.koala_base_uri)
}

pub async fn exchange_code<S: AsRef<str>>(
    config: &Config,
    code: S,
) -> Result<ExchangeResponse, reqwest::Error> {
    Client::new()
        .post(get_koala_token_url(config))
        .header("User-Agent", config.koala_user_agent())
        .header("Accept", "application/json")
        .json(&ExchangeRequest {
            grant_type: GrantType::AuthorizationCode,
            code: Some(code.as_ref()),
            client_id: &config.koala_client_id,
            client_secret: &config.koala_client_secret,
            redirect_uri: &config.koala_oauth_redirect_uri,
            refresh_token: None,
        })
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub first_name: String,
    pub infix: Option<String>,
    pub last_name: String,
}

fn get_user_info_url(config: &Config, id: i32) -> String {
    format!("{}/api/members/{id}", config.koala_base_uri)
}

pub async fn get_user_info<S: AsRef<str>>(
    config: &Config,
    access_token: S,
    koala_id: i32,
) -> Result<UserInfo, reqwest::Error> {
    Client::new()
        .get(get_user_info_url(config, koala_id))
        .header("User-Agent", config.koala_user_agent())
        .bearer_auth(access_token.as_ref())
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}

#[allow(unused)]
pub async fn get_new_access_token<S: AsRef<str>>(
    config: &Config,
    refresh_token: S,
) -> Result<ExchangeResponse, reqwest::Error> {
    Client::new()
        .post(get_koala_token_url(config))
        .header("User-Agent", config.koala_user_agent())
        .header("Accept", "application/json")
        .json(&ExchangeRequest {
            grant_type: GrantType::RefreshToken,
            code: None,
            client_id: &config.koala_client_id,
            client_secret: &config.koala_client_secret,
            redirect_uri: &config.koala_oauth_redirect_uri,
            refresh_token: Some(refresh_token.as_ref()),
        })
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}

#[derive(Deserialize)]
struct OauthUserInfoResponse {
    sub: String,
}

#[derive(Debug, Error)]
pub enum UserIdFromTokenError {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    IntParse(#[from] std::num::ParseIntError),
}

fn get_token_info_url(config: &Config) -> String {
    format!("{}/oauth/userinfo", config.koala_base_uri)
}

pub async fn get_user_id_from_token<S: AsRef<str>>(config: &Config, access_token: S) -> Result<i32, UserIdFromTokenError> {
    trace!("{}", access_token.as_ref());
    let res: OauthUserInfoResponse = Client::new()
        .get(get_token_info_url(config))
        .header("User-Agent", config.koala_user_agent())
        .header("Accept", "application/json")
        .bearer_auth(access_token.as_ref())
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(res.sub.parse::<i32>()?)
}
