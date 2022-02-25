// use std::collections::HashSet;

// use crate::http::request::HttpMethod;

// use super::HttpClient;

use std::env;

use package_info::PackageInfo;

use crate::{config::CargoPackageInfo, http::Headers};

const DEFAULT_TIMEOUT: u32 = 60;

#[derive(Debug)]
pub struct HttpClientConfiguration {
    pub timeout: u32,
    pub user_agent: String,
    pub default_headers: Headers,
}

impl HttpClientConfiguration {
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

    pub(crate) fn default_user_agent() -> String {
        let sdk_version = CargoPackageInfo::version().unwrap_or_default();
        let engine = "Rust";
        let rust_version = rustc_version_runtime::version();
        let os = env::consts::OS;
        format!("Rust Square API Client Lib/0.1.0 ({}) {}/{} ({})", sdk_version, engine, rust_version, os)
    }
}

impl Default for HttpClientConfiguration {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            user_agent: Self::default_user_agent(),
            default_headers: Default::default(),
        }
    }
}
