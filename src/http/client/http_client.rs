

use std::time::Duration;

use log::error;
use reqwest::Response;
use serde::Serialize;

use crate::models::errors::ApiError;

use super::HttpClientConfiguration;

#[derive(Clone, Debug)]
pub struct HttpClient {
    pub client: reqwest::Client,
}

impl HttpClient {
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

    pub async fn post<T: Serialize>(&self, url: &str, body: &T) -> Result<Response, ApiError> {
        self.client.post(url).json(body).send().await.map_err(|e| {
            let msg = format!("Error posting: {}", e);
            error!("{}", msg);
            ApiError::new(&msg)
        })
    }
}
