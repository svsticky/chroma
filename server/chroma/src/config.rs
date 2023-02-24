use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// MySQL database host.
    /// Non-standard ports are not supported and should not be provided.
    pub db_host: String,
    /// MySQL database name
    pub db_database: String,
    /// MySQL database username
    pub db_username: String,
    /// MySQL database password
    pub db_password: String,

    /// The name of the S3 bucket that should be used
    pub s3_bucket_name: String,
    /// The S3 region the endpoint is located in
    pub s3_region: String,
    /// S3 endpoint URL
    pub s3_endpoint_url: String,
    /// S3 secret key ID
    pub s3_access_key_id: String,
    /// S3 secret access key
    pub s3_secret_access_key: String,

    /// OAuth2 client ID created in Koala
    pub koala_client_id: String,
    /// OAuth2 client secret created in Koala.
    pub koala_client_secret: String,
    /// Koala's base URL.
    /// Used for making requests to Koala.
    /// Should not end with a trailing `/`.
    /// E.g. `https://koala.svsticky.nl`
    pub koala_base_url: String,
    /// The URI to which Koala should redirect back after login.
    /// This should consist of the base url, i.e. where Chroma is accessed via the browser,
    /// with the path `/api/v1/login` appended to it.
    /// E.g. `https://chroma.example.com/api/v1/login` would be a correct value.
    pub koala_oauth_redirect_uri: String,
    /// The user agent to be sent when making requests to Koala.
    /// If not provided, the default [Config::DEFAULT_KOALA_USER_AGENT] will be used.
    koala_user_agent: Option<String>,
    /// The URI to which Chroma should redirect after the user
    /// has logged in. Refer to the UI's documentation
    /// for what this value should be.
    /// The query parameters `session_id` and `is_admin` will be appended.
    /// No existing query parameters should be in the URI.
    /// E.g. `https://foo.example.com/logged_in` will become
    /// `https://foo.example.com/logged_in?session_id={AN ID}&is_admin=[true|false]`.
    pub login_complete_redirect_uri: String
}

impl Config {
    /// The default user agent for Koala when none is configured
    const DEFAULT_KOALA_USER_AGENT: &'static str = "Chroma server";

    /// Parse the configuration from environmental variables.
    ///
    /// # Errors
    ///
    /// If not all required variables are present
    pub fn parse() -> envy::Result<Self> {
        envy::from_env()
    }

    /// Get the User-Agent to use when sending requests to Koala
    pub fn koala_user_agent(&self) -> &str {
        self.koala_user_agent.as_deref().unwrap_or(Self::DEFAULT_KOALA_USER_AGENT)
    }
}