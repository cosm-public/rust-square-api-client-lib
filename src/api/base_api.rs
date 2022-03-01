use async_trait::async_trait;
use log::{error, warn};
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::models::errors::{ApiError, ErrorResponse};


#[async_trait]
pub(crate) trait BaseApi {
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T, ApiError> {
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
