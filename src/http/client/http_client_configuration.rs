//! Configuration for HTTP client settings

use std::env;

use package_info::PackageInfo;

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
        }
    }
}
