

use std::{collections::HashMap, env};

use log::{error, warn};
use package_info::PackageInfo;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Serialize, Deserialize};

use crate::{
    models::{
        errors::{Error, ApiError, ErrorResponse},
        CreateCardRequest,
        CreateCardResponse,
    },
    // http::{
    //     client::{
    //         HttpClient,
    //         HttpContext
    //     },
    //     request::HttpRequest
    // },
    config::{Configuration, CargoPackageInfo}
};

const DEFAULT_SQUARE_VERSION: &str = "2022-02-16";
const DEFAULT_URL: &str = "https://connect.squareupsandbox.com/v2/cards";

pub struct CardsApi;

impl CardsApi {
    pub async fn create_card(&self, body: &CreateCardRequest) -> Result<CreateCardResponse, ApiError> {
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

        let client = client_builder.build().map_err(|e| {
            let msg = format!("Failed to build client: {}", e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;

        let response = client.post(DEFAULT_URL).json(body).send().await.map_err(|e| {
            let msg = format!("Error posting: {}", e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        if response.status().is_success() {
            Ok(response.json().await.map_err(|e| {
                let msg = format!("Error deserializing: {}", e);
                error!("{}", msg);
                ApiError::new(&msg)
            })?)
        } else {
            let err_response_res: Result<ErrorResponse, reqwest::Error> = response.json().await;
            match err_response_res {
                Ok(error_response) => {
                    let api_error = ApiError::with_response_errors("Error response", &error_response.errors);
                    warn!("{:?}", api_error);
                    Err(api_error)
                },
                Err(e) => {
                    let msg = format!("Error deserializing response errors: {}", e);
                    error!("{}", msg);
                    Err(ApiError::new(&msg))
                },
            }
        }
    }
}

impl Default for CardsApi {
    fn default() -> Self {
        Self
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
