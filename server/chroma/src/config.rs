use serde::Deserialize;
use tracing::info;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // ANCHOR: config
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
    /// Force the use of path style bucket addressing.
    /// This should be `true` if the S3 endpoint is MinIO,
    /// but should be `false` or left unspecified when targeting
    /// Amazon S3.
    pub s3_force_path_style: Option<bool>,

    /// OAuth2 client ID created in Koala
    pub koala_client_id: String,
    /// OAuth2 client secret created in Koala.
    pub koala_client_secret: String,
    /// Koala's base URL for redirecting a client
    /// Used to redirect clients to Koala's OAuth.
    /// Useful if the public route is different from the route from
    /// the chroma server to koala.
    /// E.g. in Docker `koala_base_uri` could be equal to `http://host.docker.internal:3000`,
    /// while `koala_base_redirect_uri` is equal to `http://koala.rails.local:3000`.
    /// Because `host.docker.internal` isn't accessible outside the container,
    /// and `koala.rails.local` isn't accessible inside the container.
    /// If left blank, the value provided in `koala_base_url` will be used.
    pub koala_base_redirect_uri: Option<String>,
    /// Koala's base URI.
    /// Used for making requests to Koala.
    /// Should not end with a trailing `/`.
    /// E.g. `https://koala.svsticky.nl`
    pub koala_base_uri: String,
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
    pub login_complete_redirect_uri: String, 
    // ANCHOR_END: config
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
        let this: Self = envy::from_env()?;

        if this.koala_base_redirect_uri.is_none() {
            info!("'koala_base_redirect_uri' is not provided, using 'koala_base_uri' instead");
        }

        if this.koala_user_agent.is_none() {
            info!(
                "'koala_user_agent' was not provided, using '{}' as a default.",
                Self::DEFAULT_KOALA_USER_AGENT
            );
        }

        Ok(this)
    }

    /// Get the User-Agent to use when sending requests to Koala
    ///
    /// See also: `koala_user_agent` fields
    pub fn koala_user_agent(&self) -> &str {
        self.koala_user_agent
            .as_deref()
            .unwrap_or(Self::DEFAULT_KOALA_USER_AGENT)
    }

    /// Koala's base URL for redirecting a client.
    ///
    /// See also: `koala_base_redirect_uri` field
    pub fn koala_base_redirect_uri(&self) -> &String {
        self.koala_base_redirect_uri
            .as_ref()
            .unwrap_or(&self.koala_base_uri)
    }

    /// Force S3 path styles instead of virtual hosts
    ///
    /// See also: `s3_force_path_style` field
    pub fn s3_force_path_style(&self) -> bool {
        self.s3_force_path_style.unwrap_or(false)
    }
}
