use std::env;

use log::error;
use package_info::PackageInfo;
use reqwest::{Client, header::{HeaderValue, HeaderMap}};

use crate::{models::errors::ApiError, config::CargoPackageInfo};


const DEFAULT_SQUARE_VERSION: &str = "2022-02-16";

pub(crate) trait BaseApi {
    fn get_client(&self) -> Result<Client, ApiError> {
        let user_agent = user_agent();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Square-Version", HeaderValue::from_static(DEFAULT_SQUARE_VERSION));
        headers.insert("accept", HeaderValue::from_static("application/json"));

        match HeaderValue::from_str(&user_agent) {
            Ok(hv) => {
                headers.insert("user-agent", hv);
            },
            Err(e) => {
                error!("Error generating user-agent header: {}", e);
                // swallow this error, since it's not critical and shouldn't happen anyway
            }
        }

        match HeaderValue::from_str(&authorization()) {
            Ok(hv) => {
                headers.insert("Authorization", hv);
            },
            Err(e) => {
                let msg = format!("Error generating Authorization header: {}", e);
                error!("{}", msg);
                return Err(ApiError::new(&msg))
            }
        }

        let mut client_builder = reqwest::ClientBuilder::new();
        client_builder = client_builder.user_agent(user_agent);
        client_builder = client_builder.default_headers(headers);

        client_builder.build().map_err(|e| {
            let msg = format!("Failed to build client: {}", e);
            error!("{}", msg);
            ApiError::new(&msg)
        })
    }
}

fn user_agent() -> String {
    let sdk_version = CargoPackageInfo::version().unwrap_or_default();
    let engine = "Rust";
    let rust_version = rustc_version_runtime::version();
    let os = env::consts::OS;
    format!("Rust Square API Client Lib/0.1.0 ({}) {}/{} ({})", sdk_version, engine, rust_version, os)
}

fn authorization() -> String {
    format!("Bearer {}", env::var("AUTH_TOKEN").unwrap())
}
