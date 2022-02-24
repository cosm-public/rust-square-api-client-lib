use log::{error, warn};

use crate::models::{CreateCustomerRequest, CreateCustomerResponse, errors::{ApiError, ErrorResponse}};

use super::BaseApi;

const DEFAULT_URL: &str = "https://connect.squareupsandbox.com/v2/customers";


pub struct CustomersApi;

impl CustomersApi {
    pub async fn create_customer(&self, body: &CreateCustomerRequest) -> Result<CreateCustomerResponse, ApiError> {
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

impl BaseApi for CustomersApi {}

impl Default for CustomersApi {
    fn default() -> Self {
        Self
    }
}
