//! HTTP Client to send HTTP Requests and read the responses.

use std::time::Duration;

use log::error;
use serde::Serialize;

use crate::{models::errors::ApiError, http::HttpResponse};

use super::HttpClientConfiguration;

/// HTTP Client to send HTTP Requests and read the responses.
#[derive(Clone, Debug)]
pub struct HttpClient {
    /// The wrapped lib client
    pub client: reqwest::Client,
}

impl HttpClient {
    /// Instantiates a new `HttpClient` given the provided `HttpClientConfiguration`.
    pub fn try_new(config: &HttpClientConfiguration) -> Result<Self, ApiError> {
        let mut client_builder = reqwest::ClientBuilder::new();
        client_builder = client_builder.timeout(Duration::from_secs(config.timeout.into()));
        client_builder = client_builder.user_agent(&config.user_agent);
        client_builder = client_builder.default_headers((&config.default_headers).try_into()?);

        let client = client_builder.build().map_err(|e| {
            let msg = format!("Failed to build client: {}", e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        
        Ok(Self { client })
    }

    /// Sends an HTTP POST
    pub async fn post<T: Serialize>(&self, url: &str, body: &T) -> Result<HttpResponse, ApiError> {
        let response = self.client.post(url).json(body).send().await.map_err(|e| {
            let msg = format!("Error posting to {}: {}", url, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        Ok(HttpResponse::new(response))
    }

    /// Sends an HTTP GET
    pub async fn get(&self, url: &str) -> Result<HttpResponse, ApiError> {
        let response = self.client.get(url).send().await.map_err(|e| {
            let msg = format!("Error getting {}: {}", url, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        Ok(HttpResponse::new(response))
    }
}
