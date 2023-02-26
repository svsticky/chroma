use serde::{Deserialize, Serialize};
use crate::config::Config;

/// Get the URL to redirect a client to when they should log in with Koala
pub fn get_koala_login_url(config: &Config) -> String {
    format!("{}/api/oauth/authorize?client_id={}&redirect_uri={}&response_type=code",
        config.koala_base_redirect_uri(),
        config.koala_client_id,
        config.koala_oauth_redirect_uri,
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
    pub credentials_id: u32,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum CredentialsType {
    Admin,
    Member,
}

#[derive(Debug, Serialize)]
struct ExchangeRequest<'a> {
    grant_type: GrantType,
    code: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    redirect_uri: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum GrantType {
    /// OAuth authorization_code flow
    AuthorizationCode
}

/// Get the URl to use to exchange an access code for an OAuth token pair,
/// or to refresh an existing access token for a new one.
fn get_koala_token_url(config: &Config) -> String {
    format!("{}/api/oauth/token", config.koala_base_uri)
}

pub async fn exchange_code<S: AsRef<str>>(config: &Config, code: S) -> Result<ExchangeResponse, reqwest::Error> {
    reqwest::Client::new()
        .post(get_koala_token_url(config))
        .header("User-Agent", config.koala_user_agent())
        .header("Accept", "application/json")
        .json(&ExchangeRequest {
            grant_type: GrantType::AuthorizationCode,
            code: code.as_ref(),
            client_id: &config.koala_client_id,
            client_secret: &config.koala_client_secret,
            redirect_uri: &config.koala_oauth_redirect_uri,
        })
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}