use crate::{models::{CreateCustomerRequest, CreateCustomerResponse, errors::ApiError}, config::Configuration, http::client::HttpClient};

use super::BaseApi;

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

        self.handle_response(response).await
    }

    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}

impl BaseApi for CustomersApi {}
