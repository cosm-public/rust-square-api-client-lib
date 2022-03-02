//! Shared API behavior

use async_trait::async_trait;
use log::{error, warn};
use serde::de::DeserializeOwned;

use crate::{models::errors::{ApiError, ErrorResponse}, http::HttpResponse};

/// All APIs can make use of base API default implementations
#[async_trait]
pub(crate) trait BaseApi {
    /// Handles API responses, including error responses
    async fn handle_response<T: DeserializeOwned>(&self, response: HttpResponse) -> Result<T, ApiError> {
        if response.is_success() {
            Ok(response.json().await?)
        } else {
            let err_response_res: Result<ErrorResponse, ApiError> = response.json().await;
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
