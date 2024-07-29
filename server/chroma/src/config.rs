use anyhow::{Error, Result};
use serde::Deserialize;
use tracing::{info, warn};

use dal::database::DbConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // ANCHOR: config
    /// Database host.
    /// Non-standard ports are not supported and should not be provided.
    pub db_host: Option<String>,
    /// Database name
    pub db_database: Option<String>,
    /// Database username
    pub db_username: Option<String>,
    /// Database password
    pub db_password: Option<String>,
    /// Database connection url
    pub db_url: Option<String>,

    /// The name of the S3 bucket that should be used
    /// Required if `storage_engine` is set to [StorageEngine::S3].
    pub s3_bucket_name: Option<String>,
    /// The S3 region the endpoint is located in
    /// Required if `storage_engine` is set to [StorageEngine::S3].
    pub s3_region: Option<String>,
    /// S3 endpoint URL
    /// Required if `storage_engine` is set to [StorageEngine::S3].
    pub s3_endpoint_url: Option<String>,
    /// S3 secret key ID
    /// Required if `storage_engine` is set to [StorageEngine::S3].
    pub s3_access_key_id: Option<String>,
    /// S3 secret access key
    /// Required if `storage_engine` is set to [StorageEngine::S3].
    pub s3_secret_access_key: Option<String>,
    /// Force the use of path style bucket addressing.
    /// This should be `true` if the S3 endpoint is MinIO,
    /// but should be `false` or left unspecified when targeting
    /// Amazon S3.
    pub s3_force_path_style: Option<bool>,
    /// Create a bucket on startup. This should only
    /// be used when working with MinIO.
    /// The provided access key should have bucket creation privileges.
    pub s3_create_bucket_on_startup: Option<bool>,

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
    /// has logged in. Refer to the UIs documentation
    /// for what this value should be.
    /// The query parameters `session_id` and `is_admin` will be appended.
    /// No existing query parameters should be in the URI.
    /// E.g. `https://foo.example.com/logged_in` will become
    /// `https://foo.example.com/logged_in?session_id={AN ID}&is_admin=[true|false]`.
    pub login_complete_redirect_uri: String,

    /// Comma-seperated list of service tokens.
    /// When making a request, use the following for the `Authorization` header:
    /// ```
    /// Authorization: Service <YOUR TOKEN>
    /// ```
    ///
    /// # Warning
    /// Service tokens can access all APIs, even admin ones!
    pub service_tokens: String,
    // ANCHOR_END: config
}

impl Config {
    /// The default user agent for Koala when none is configured
    const DEFAULT_KOALA_USER_AGENT: &'static str = "Chroma server";

    pub fn oauth_client_config(&self) -> cabbage::oauth::ClientConfig {
        cabbage::oauth::ClientConfig::new(
            self.koala_client_id.clone(),
            self.koala_client_secret.clone(),
            self.koala_oauth_redirect_uri.clone(),
        )
    }

    /// Get the database configuration specified by the configuration
    pub fn database_config(&self) -> Result<DbConfig> {
        if let Some(url) = &self.db_url {
            Ok(DbConfig::Url { url })
        } else {
            match (&self.db_host, &self.db_username, &self.db_password, &self.db_database) {
                (Some(host), Some(user), Some(passw), Some(database)) => Ok(DbConfig::Parameters { host, user, passw, database }),
                _ => Err(Error::msg("Database is configured incorrectly. You must specify either a `db_url` OR `db_host`, `db_username`, `db_password` and `db_database`"))
            }
        }
    }

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
    #[allow(unused)] // TODO
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

    /// Force S3 path styles instead of virtual hosts.
    ///
    /// See also: `s3_force_path_style` field.
    pub fn s3_force_path_style(&self) -> bool {
        self.s3_force_path_style.unwrap_or(false)
    }

    /// Create an S3 bucket on application startup.
    ///
    /// See also: `s3_create_bucket_on_startup` field.
    pub fn s3_create_bucket_on_startup(&self) -> bool {
        self.s3_create_bucket_on_startup.unwrap_or(false)
    }

    /// Get configured service tokens
    pub fn service_tokens(&self) -> Vec<&str> {
        self.service_tokens.split(',').collect()
    }

    /// Check if the configuration is valid.
    /// Returns `true` if it is, `false` if it is not.
    pub fn validate(&self) -> bool {
        let check_field = |field_name: &'static str, field_value: &Option<_>| {
            if field_value.is_none() {
                warn!("Config validation failed on S3_{field_name}");
                false
            } else {
                true
            }
        };

        let config_ok = check_field("ACCESS_KEY_ID", &self.s3_access_key_id)
            && check_field("SECRET_ACCESS_KEY", &self.s3_secret_access_key)
            && check_field("BUCKET_NAME", &self.s3_bucket_name)
            && check_field("ENDPOINT_URL", &self.s3_endpoint_url)
            && check_field("REGION", &self.s3_region);

        if !config_ok {
            warn!("Config validation failed.");
            return false;
        }

        true
    }
}
