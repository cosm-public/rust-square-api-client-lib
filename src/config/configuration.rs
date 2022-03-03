//! App configuration for the library

use std::env;

use log::warn;

use crate::http::client::HttpClientConfiguration;

use super::Environment;

const DEFAULT_BASE_URI: &str = "/v2";
const DEFAULT_SQUARE_VERSION: &str = "2022-02-16";

/// Configuration struct for the library
#[derive(Clone, Debug)]
pub struct Configuration {
    /// Current API environment
    pub environment: Environment,
    /// Base URL requests are made to. Defaults to `https://connect.squareup.com` and
    /// `https://connect.squareupsandbox.com` for Prod and Sandbox environments respectively,
    /// and only read from here if `environment` is `Custom`
    pub custom_url: Option<String>,
    /// Square connect API versions.
    pub square_version: String,
    /// Http Client Configuration instance.
    pub http_client_config: HttpClientConfiguration,
    /// OAuth 2.0 Access Token, if this isn't provided during initialization, environment
    /// variable SQUARE_API_TOKEN is checked. Failure to provide an API token by one of
    /// these methods will result in unauthorized requests
    pub access_token: String,
    /// Base URI
    pub base_uri: String,
}

impl Configuration {
    /// Gets the base Square API URL for the configured environment, including the API version
    /// specifier (e.g. "/v2")
    pub(crate) fn get_base_url(&self) -> String {
        let base_url = match self.environment.get_base_url() {
            Some(base_url) => base_url,
            None => match &self.custom_url {
                Some(base_url) => base_url.as_str(),
                None => Environment::Sandbox.get_base_url().unwrap(),
            },
        };

        format!("{}{}", base_url, self.base_uri)
    }

    /// The default authorization header is a Bearer token found in the `SQUARE_API_TOKEN`
    /// environment variable
    pub(crate) fn default_authorization() -> String {
        format!(
            "Bearer {}",
            env::var("SQUARE_API_TOKEN").unwrap_or_else(|_| {
                warn!("No SQUARE_API_TOKEN environment variable found");
                String::new()
            })
        )
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            environment: Default::default(),
            custom_url: Default::default(),
            square_version: DEFAULT_SQUARE_VERSION.to_owned(),
            http_client_config: Default::default(),
            access_token: Self::default_authorization(),
            base_uri: DEFAULT_BASE_URI.to_owned(),
        }
    }
}
