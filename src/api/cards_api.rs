

use crate::{
    models::{
        errors::ApiError,
        CreateCardRequest,
        CreateCardResponse,
    }, config::Configuration, http::client::HttpClient,
};

use super::BaseApi;

const DEFAULT_URI: &str = "/cards";

pub struct CardsApi {
    config: Configuration,
    client: HttpClient,
}

impl CardsApi {
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    pub async fn create_card(&self, body: &CreateCardRequest) -> Result<CreateCardResponse, ApiError> {
        let response = self.client.post(&self.url(), body).await?;
        
        self.handle_response(response).await
    }

    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}

impl BaseApi for CardsApi {}
