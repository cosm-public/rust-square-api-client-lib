//! Use the Cards API to save a credit or debit card on file.
//!
//! You can use the CreateCard endpoint to save a credit or debit card to a Square account.
//! Developers can integrate the Cards API in their application to let Square sellers:
//!
//! * Save a card that can be charged by any Square seller who uses your application. Your
//! application specifies the organization access token in the CreateCard request.
//! * Save a card that can be charged by a single Square seller. Your application specifies the
//! access token of the specific seller account in the CreateCard request.
//!
//! The Cards API also supports other endpoints to manage the cards.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::ApiError, CreateCardRequest, CreateCardResponse, DisableCardResponse,
        ListCardsParameters, ListCardsResponse, RetrieveCardResponse,
    },
};

use super::BaseApi;

const DEFAULT_URI: &str = "/cards";

/// Use the Cards API to save a credit or debit card on file.
pub struct CardsApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Cards API endpoints
    client: HttpClient,
}

impl CardsApi {
    /// Instantiates a new `CardsApi`
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Adds a card on file to an existing merchant.
    pub async fn create_card(
        &self,
        body: &CreateCardRequest,
    ) -> Result<CreateCardResponse, ApiError> {
        let response = self.client.post(&self.url(), body).await?;

        self.handle_response(response).await
    }

    /// Disables the card, preventing any further updates or charges.
    /// Disabling an already disabled card is allowed but has no effect.
    pub async fn disable_card(&self, card_id: &str) -> Result<DisableCardResponse, ApiError> {
        let url = format!("{}/{}/disable", &self.url(), card_id);
        let response = self.client.post(&url, &String::new()).await?;

        self.handle_response(response).await
    }

    /// Retrieves a list of cards owned by the account making the request.
    /// A max of 25 cards will be returned.
    pub async fn list_cards(
        &self,
        params: &ListCardsParameters,
    ) -> Result<ListCardsResponse, ApiError> {
        let url = format!("{}{}", &self.url(), params.to_query_string());
        let response = self.client.get(&url).await?;

        self.handle_response(response).await
    }

    /// Retrieves details for a specific Card.
    pub async fn retrieve_card(&self, card_id: &str) -> Result<RetrieveCardResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), card_id);
        let response = self.client.get(&url).await?;

        self.handle_response(response).await
    }

    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}

impl BaseApi for CardsApi {}
