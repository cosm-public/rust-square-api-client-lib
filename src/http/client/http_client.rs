//! HTTP Client to send HTTP Requests and read the responses.

use std::time::Duration;

use log::error;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use serde::Serialize;

use crate::http::client::http_client_configuration::RetryConfiguration;
use crate::{http::HttpResponse, models::errors::ApiError};

use super::HttpClientConfiguration;

/// HTTP Client to send HTTP Requests and read the responses.
#[derive(Clone, Debug)]
pub struct HttpClient {
    /// The wrapped lib client
    pub client: ClientWithMiddleware,
}

impl HttpClient {
    /// Instantiates a new `HttpClient` given the provided `HttpClientConfiguration`.
    pub fn try_new(config: &HttpClientConfiguration) -> Result<Self, ApiError> {
        let mut client_builder = reqwest::ClientBuilder::new();
        client_builder = client_builder.timeout(Duration::from_secs(config.timeout.into()));
        client_builder = client_builder.user_agent(&config.user_agent);
        client_builder = client_builder.default_headers((&config.default_headers).try_into()?);
        let retry_policy = create_retry_policy(&config.retry_configuration);
        let client = ClientBuilder::new(client_builder.build().map_err(|e| {
            let msg = format!("Failed to build client: {}", e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
        Ok(Self { client })
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

    /// Sends an HTTP POST
    pub async fn post<T: Serialize>(&self, url: &str, body: &T) -> Result<HttpResponse, ApiError> {
        let response = self.client.post(url).json(body).send().await.map_err(|e| {
            let msg = format!("Error posting to {}: {}", url, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        Ok(HttpResponse::new(response))
    }

    /// Sends an HTTP PUT
    pub async fn put<T: Serialize>(&self, url: &str, body: &T) -> Result<HttpResponse, ApiError> {
        let response = self.client.put(url).json(body).send().await.map_err(|e| {
            let msg = format!("Error putting to {}: {}", url, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        Ok(HttpResponse::new(response))
    }

    /// Sends an HTTP DELETE
    pub async fn delete(&self, url: &str) -> Result<HttpResponse, ApiError> {
        let response = self.client.delete(url).send().await.map_err(|e| {
            let msg = format!("Error putting to {}: {}", url, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        Ok(HttpResponse::new(response))
    }
}

fn create_retry_policy(retry_configuration: &RetryConfiguration) -> ExponentialBackoff {
    let mut retry_policy =
        ExponentialBackoff::builder().build_with_max_retries(retry_configuration.retries_count);
    retry_policy.max_retry_interval = retry_configuration.max_retry_interval;
    retry_policy.min_retry_interval = retry_configuration.min_retry_interval;
    retry_policy.backoff_exponent = retry_configuration.backoff_exponent;
    retry_policy
}

#[cfg(test)]
mod tests {
    use crate::http::client::{HttpClient, HttpClientConfiguration};

    #[test]
    fn try_new_ok() {
        let client = HttpClient::try_new(&HttpClientConfiguration::default());
        assert!(client.is_ok());
    }
}
