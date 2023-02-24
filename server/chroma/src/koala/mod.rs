use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::config::Config;

/// Get the URL to redirect a client to when they should log in with Koala
pub fn get_koala_login_url(config: &Config) -> String {
    format!("{}/api/oauth/authorize?client_id={}&redirect_uri={}&response_type=code",
        config.koala_base_url,
        config.koala_client_id,
        config.koala_oauth_redirect_uri,
    )
}

#[derive(Debug, Deserialize)]
pub struct ExchangeResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub created_at: i64,
    #[serde(flatten)]
    pub member: MemberInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MemberInfo {
    Admin(Member),
    Member(Member),
}

#[derive(Debug, Deserialize)]
pub struct Member {
    pub id: u32,
    pub name: String,
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
    format!("{}/api/oauth/token", config.koala_base_url)
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

#[cfg(test)]
mod test {
    use crate::koala::{ExchangeResponse, MemberInfo};

    #[test]
    fn test_exchange_response_member() {
        // From Koala docs
        let json = r#"
            {
              "access_token": "eb49949219182ce572529fec8be863af2c1847061de1db21e4304f615f66c04b",
              "token_type": "bearer",
              "expires_in": 14400,
              "refresh_token": "d247c9b2047e7711ad2f0b6b2b086944b02f4e1f659f1076a3670a5d611a5d4f",
              "scope": "member-read activity-read group-read",
              "created_at": 1446895075,
              "member": {
                "id": 1,
                "name": "Martijn Casteel",
                "email": "martijn.casteel@gmail.com"
              }
            }
        "#;

        let deserialized = serde_json::from_str::<ExchangeResponse>(json);
        assert!(deserialized.is_ok());

        let info = deserialized.unwrap();
        assert_eq!(info.access_token, "eb49949219182ce572529fec8be863af2c1847061de1db21e4304f615f66c04b");

        let member = match info.member {
            MemberInfo::Member(v) => v,
            MemberInfo::Admin(_) => panic!("Expected MemberInfo variant Member, got variant Admin"),
        };

        assert_eq!(member.id, 1);
    }

    #[test]
    fn test_exchange_response_admin() {
        // From Koala docs
        let json = r#"
            {
              "access_token": "eb49949219182ce572529fec8be863af2c1847061de1db21e4304f615f66c04b",
              "token_type": "bearer",
              "expires_in": 14400,
              "refresh_token": "d247c9b2047e7711ad2f0b6b2b086944b02f4e1f659f1076a3670a5d611a5d4f",
              "scope": "member-read activity-read group-read",
              "created_at": 1446895075,
              "admin": {
                "id": 1,
                "name": "Martijn Casteel",
                "email": "martijn.casteel@gmail.com"
              }
            }
        "#;

        let deserialized = serde_json::from_str::<ExchangeResponse>(json);
        assert!(deserialized.is_ok());

        let info = deserialized.unwrap();
        assert_eq!(info.access_token, "eb49949219182ce572529fec8be863af2c1847061de1db21e4304f615f66c04b");

        let member = match info.member {
            MemberInfo::Admin(v) => v,
            MemberInfo::Member(_) => panic!("Expected MemberInfo variant Admin, got variant Member"),
        };

        assert_eq!(member.id, 1);
    }
}