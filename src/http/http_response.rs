//! Representation of HTTP API response

use log::error;
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::models::errors::ApiError;

/// Representation of HTTP API response.
///
/// Wraps Reqwest lib's Response
pub struct HttpResponse {
    inner: Response,
}

impl HttpResponse {
    pub fn new(inner: Response) -> Self {
        Self { inner }
    }

    pub fn is_success(&self) -> bool {
        self.inner.status().is_success()
    }

    pub async fn json<T: DeserializeOwned>(self) -> Result<T, ApiError> {
        self.inner.json().await.map_err(|e| {
            let msg = format!("Error deserializing: {}", e);
            error!("{}", msg);
            ApiError::new(&msg)
        })
    }
}
