use log::{error, warn};

use crate::{models::{CreateCustomerRequest, CreateCustomerResponse, errors::{ApiError, ErrorResponse}}, config::Configuration, http::client::HttpClient};

const DEFAULT_URI: &str = "/customers";


pub struct CustomersApi {
    config: Configuration,
    client: HttpClient,
}

impl CustomersApi {
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }
    
    pub async fn create_customer(&self, body: &CreateCustomerRequest) -> Result<CreateCustomerResponse, ApiError> {
        let response = self.client.post(&self.url(), body).await?;

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

    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
