//! HTTP Client to send HTTP Requests and read the responses.

use std::fs::File;
use std::io::Read;
use std::{fmt::Debug, time::Duration};

use log::error;
use reqwest::multipart::{self, Part};
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
    pub client: reqwest::Client,
    pub retry_client: ClientWithMiddleware,
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
        let retry_policy = create_retry_policy(&config.retry_configuration);
        let retry_client = ClientBuilder::new(client.clone())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        Ok(Self {
            client,
            retry_client,
        })
    }

    /// Sends an HTTP GET
    pub async fn get(&self, url: &str) -> Result<HttpResponse, ApiError> {
        let response = self.retry_client.get(url).send().await.map_err(|e| {
            let msg = format!("Error getting {}: {}", url, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        Ok(HttpResponse::new(response))
    }

    /// Sends an HTTP POST
    pub async fn post<T: Serialize + ?Sized>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<HttpResponse, ApiError> {
        let response = self.retry_client.post(url).json(body).send().await.map_err(|e| {
            let msg = format!("Error posting to {}: {}", url, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        Ok(HttpResponse::new(response))
    }

    /// Sends an HTTP POST with multipart form data
    pub async fn post_multipart<T: Debug + Serialize>(
        &self,
        url: &str,
        body: &T,
        filepath: &str,
    ) -> Result<HttpResponse, ApiError> {
        let request = serde_json::to_string(body).map_err(|e| {
            let msg =
                format!("Error serializing request body - url: {}, body: {:?}: {}", url, body, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;

        let mut file = File::open(filepath).map_err(|e| {
            let msg = format!("Error opening file {}: {}", filepath, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        let mut vec = Vec::new();
        let _reader = file.read_to_end(&mut vec);
        let mime = get_mime_type(filepath)?;
        let part = Part::stream(vec).mime_str(mime).map_err(|e| {
            let msg = format!(
                "Error applying content type {} to form part for file {}: {}",
                mime, filepath, e
            );
            error!("{}", msg);
            ApiError::new(&msg)
        })?;

        let form = multipart::Form::new().text("request", request).part("file", part);

        let response = self.client.post(url).multipart(form).send().await.map_err(|e| {
            let msg = format!("Error posting to {}: {}", url, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        Ok(HttpResponse::new(response))
    }

    /// Sends an HTTP POST without any body
    pub async fn empty_post(&self, url: &str) -> Result<HttpResponse, ApiError> {
        let response = self.client.post(url).send().await.map_err(|e| {
            let msg = format!("Error posting to {}: {}", url, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        Ok(HttpResponse::new(response))
    }

    /// Sends an HTTP PUT
    pub async fn put<T: Serialize>(&self, url: &str, body: &T) -> Result<HttpResponse, ApiError> {
        let response = self.retry_client.put(url).json(body).send().await.map_err(|e| {
            let msg = format!("Error putting to {}: {}", url, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        Ok(HttpResponse::new(response))
    }

    /// Sends an HTTP PUT with multipart form data
    pub async fn put_multipart<T: Debug + Serialize>(
        &self,
        url: &str,
        body: &T,
        filepath: &str,
    ) -> Result<HttpResponse, ApiError> {
        let request = serde_json::to_string(body).map_err(|e| {
            let msg =
                format!("Error serializing request body - url: {}, body: {:?}: {}", url, body, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;

        let mut file = File::open(filepath).map_err(|e| {
            let msg = format!("Error opening file {}: {}", filepath, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        let mut vec = Vec::new();
        let _reader = file.read_to_end(&mut vec);
        let mime = get_mime_type(filepath)?;
        let part = Part::stream(vec).mime_str(mime).map_err(|e| {
            let msg = format!(
                "Error applying content type {} to form part for file {}: {}",
                mime, filepath, e
            );
            error!("{}", msg);
            ApiError::new(&msg)
        })?;

        let form = multipart::Form::new().text("request", request).part("file", part);

        let response = self.client.put(url).multipart(form).send().await.map_err(|e| {
            let msg = format!("Error putting to {}: {}", url, e);
            error!("{}", msg);
            ApiError::new(&msg)
        })?;
        Ok(HttpResponse::new(response))
    }

    /// Sends an HTTP DELETE
    pub async fn delete(&self, url: &str) -> Result<HttpResponse, ApiError> {
        let response = self.retry_client.delete(url).send().await.map_err(|e| {
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

/// Tries to determine the file's MIME type and returns it as a str
fn get_mime_type(filepath: &str) -> Result<&str, ApiError> {
    let kind = infer::get_from_path(filepath).map_err(|e| {
        let msg = format!("Error reading file {}: {}", filepath, e);
        error!("{}", msg);
        ApiError::new(&msg)
    })?;

    match kind {
        Some(kind) => Ok(kind.mime_type()),
        None => {
            let msg = format!("Error determining mime type for file {}", filepath);
            error!("{}", msg);
            Err(ApiError::new(&msg))
        }
    }
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
