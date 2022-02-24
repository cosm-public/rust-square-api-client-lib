

use log::{error, warn};

use crate::{
    models::{
        errors::{ApiError, ErrorResponse},
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
};

use super::BaseApi;

const DEFAULT_URL: &str = "https://connect.squareupsandbox.com/v2/cards";

pub struct CardsApi;

impl CardsApi {
    pub async fn create_card(&self, body: &CreateCardRequest) -> Result<CreateCardResponse, ApiError> {
        let response = self.get_client()?.post(DEFAULT_URL, body).await?;
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

impl BaseApi for CardsApi {}

impl Default for CardsApi {
    fn default() -> Self {
        Self
    }
}
