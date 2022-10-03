//! Create and retrieve gift card activities.
//!
//! Developers can use the Gift Cards API to create and access gift cards and use the Gift Card
//! Activities API to create gift card activities, such as activate a gift card, add funds to a gift
//! card, and redeem a gift card.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::ApiError, CreateGiftCardActivityRequest, CreateGiftCardActivityResponse,
        ListGiftCardActivitiesParameters, ListGiftCardActivitiesResponse,
    },
};

const DEFAULT_URI: &str = "/gift-cards/activities";

pub struct GiftCardActivitiesApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Gift Card Activities API endpoints
    client: HttpClient,
}

impl GiftCardActivitiesApi {
    /// Instantiates a new `GiftCardActivitiesApi`
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Lists gift card activities.
    ///
    /// By default, you get gift card activities for all gift cards in the seller's account. You can
    /// optionally specify query parameters to filter the list. For example, you can get a list of
    /// gift card activities for a gift card, for all gift cards in a specific region, or for
    /// activities within a time window.
    pub async fn list_gift_card_activities(
        &self,
        params: &ListGiftCardActivitiesParameters,
    ) -> Result<ListGiftCardActivitiesResponse, ApiError> {
        let url = format!("{}{}", &self.url(), params.to_query_string());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Creates a gift card activity to manage the balance or state of a [gift card](GiftCard).
    ///
    /// For example, you create an `ACTIVATE` activity to activate a gift card with an initial
    /// balance before the gift card can be used.
    pub async fn create_gift_card_activity(
        &self,
        body: &CreateGiftCardActivityRequest,
    ) -> Result<CreateGiftCardActivityResponse, ApiError> {
        let response = self.client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
