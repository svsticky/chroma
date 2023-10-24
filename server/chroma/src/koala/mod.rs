use crate::config::Config;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserNameInfo {
    pub first_name: String,
    pub infix: Option<String>,
    pub last_name: String,
}

fn get_member_info_url(config: &Config, id: i32) -> String {
    format!("{}/api/members/{id}", config.koala_base_uri)
}

pub async fn get_member<S: AsRef<str>>(
    config: &Config,
    access_token: S,
    koala_id: i32,
) -> Result<UserNameInfo, reqwest::Error> {
    Client::new()
        .get(get_member_info_url(config, koala_id))
        .header("User-Agent", config.koala_user_agent())
        .bearer_auth(access_token.as_ref())
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}
