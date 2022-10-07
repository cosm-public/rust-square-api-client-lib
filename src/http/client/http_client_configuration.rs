//! Configuration for HTTP client settings

use package_info::PackageInfo;
use std::default::Default;
use std::env;
use std::time::Duration;

use crate::{config::CargoPackageInfo, http::Headers};

const DEFAULT_TIMEOUT: u32 = 60;

/// Configuration for HTTP client settings
#[derive(Clone, Debug)]
pub struct HttpClientConfiguration {
    /// Timeout for HTTP connections
    pub timeout: u32,
    /// User Agent to use for requests
    pub user_agent: String,
    /// Headers to send with each request
    pub default_headers: Headers,
    /// Retry mechanism configuration
    pub retry_configuration: RetryConfiguration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetryConfiguration {
    /// How many times client should call same request in case of failure.
    pub retries_count: u32,
    /// Minimum waiting time between two retry attempts (it can end up being lower due to jittering).
    pub min_retry_interval: Duration,
    /// Maximum waiting time between two retry attempts.
    pub max_retry_interval: Duration,
    /// Growing factor governing how fast the retry interval increases with respect to the number
    /// of failed attempts. If set to 3:
    /// - first retry: 3^0 = 1
    /// - second retry: 3^1 = 3
    /// - third retry: 3^2 = 9
    pub backoff_exponent: u32,
}

impl Default for RetryConfiguration {
    fn default() -> Self {
        Self {
            retries_count: 0,
            min_retry_interval: Duration::from_secs(1),
            max_retry_interval: Duration::from_secs(30 * 60),
            backoff_exponent: 3,
        }
    }
}

impl HttpClientConfiguration {
    /// Instantiates a new `HttpClientConfiguration` based on the specified settings.
    /// If the User Agent is not included in the default headers, it'll be added.
    pub fn new(timeout: u32, user_agent: String, mut default_headers: Headers) -> Self {
        if !default_headers.has_user_agent() {
            default_headers.set_user_agent(&user_agent);
        }
        Self {
            timeout,
            user_agent,
            default_headers,
            retry_configuration: Default::default(),
        }
    }

    /// Provides the library/crate's default User Agent
    pub(crate) fn default_user_agent() -> String {
        let sdk_version = CargoPackageInfo::version().unwrap_or_default();
        let engine = "Rust";
        let rust_version = rustc_version_runtime::version();
        let os = env::consts::OS;
        format!(
            "Rust Square API Client Lib/0.1.0 ({}) {}/{} ({})",
            sdk_version, engine, rust_version, os
        )
    }
}

impl Default for HttpClientConfiguration {
    /// The default HTTP client settings
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            user_agent: Self::default_user_agent(),
            default_headers: Default::default(),
            retry_configuration: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http::client::{HttpClientConfiguration, RetryConfiguration};
    use crate::http::Headers;

    #[test]
    fn http_client_configuration_new_with_default_headers() {
        let http_client_configuration =
            HttpClientConfiguration::new(15, String::from("some_user_agent"), Headers::default());
        assert_eq!(15, http_client_configuration.timeout);
        assert_eq!(String::from("some_user_agent"), http_client_configuration.user_agent);
        assert_eq!(Headers::default(), http_client_configuration.default_headers);
        assert_eq!(RetryConfiguration::default(), http_client_configuration.retry_configuration);
    }

    #[test]
    fn http_client_configuration_new_with_different_user_agent_in_headers() {
        // Headers::
        let http_client_configuration =
            HttpClientConfiguration::new(15, String::from("some_user_agent"), Headers::default());
        assert_eq!(15, http_client_configuration.timeout);
        assert_eq!(String::from("some_user_agent"), http_client_configuration.user_agent);
        assert_eq!(Headers::default(), http_client_configuration.default_headers);
        assert_eq!(RetryConfiguration::default(), http_client_configuration.retry_configuration);
    }
}
