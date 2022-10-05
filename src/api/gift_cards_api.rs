//! Create and access gift cards and link customers to gift cards.
//!
//! Square gift cards enable sellers to boost sales and attract new customers. Sellers can easily
//! sell, redeem, track, and reload Square gift cards.
//!
//! Developers can use the Gift Cards API to integrate Square gift cards into third-party
//! applications. In addition, developers can use the Gift Card Activities API to create gift card
//! activities, such as activate a gift card, add funds to a gift card, and redeem a gift card.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::ApiError, CreateGiftCardRequest, CreateGiftCardResponse,
        LinkCustomerToGiftCardRequest, LinkCustomerToGiftCardResponse, ListGiftCardsParameters,
        ListGiftCardsResponse, RetrieveGiftCardFromGANRequest, RetrieveGiftCardFromGANResponse,
        RetrieveGiftCardFromNonceRequest, RetrieveGiftCardFromNonceResponse,
        RetrieveGiftCardResponse, UnlinkCustomerFromGiftCardRequest,
        UnlinkCustomerFromGiftCardResponse,
    },
};

const DEFAULT_URI: &str = "/gift-cards";

pub struct GiftCardsApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Gift Cards API endpoints
    client: HttpClient,
}

impl GiftCardsApi {
    /// Instantiates a new `GiftCardsApi`
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Lists all gift cards.
    ///
    /// You can specify optional filters to retrieve a subset of the gift cards. Results are sorted
    /// by created_at in ascending order.
    pub async fn list_gift_cards(
        &self,
        params: &ListGiftCardsParameters,
    ) -> Result<ListGiftCardsResponse, ApiError> {
        let url = format!("{}{}", &self.url(), params.to_query_string());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Creates a digital gift card or registers a physical (plastic) gift card.
    ///
    /// After the gift card is created, you must call
    /// [CreateGiftCardActivity](https://developer.squareup.com/reference/square/giftcardactivities-api/create-gift-card-activity)
    /// to activate the card with an initial balance before it can be used for payment.
    pub async fn create_gift_card(
        &self,
        body: &CreateGiftCardRequest,
    ) -> Result<CreateGiftCardResponse, ApiError> {
        let response = self.client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Retrieves a gift card using the gift card account number (GAN).
    pub async fn retrieve_gift_card_from_gan(
        &self,
        body: &RetrieveGiftCardFromGANRequest,
    ) -> Result<RetrieveGiftCardFromGANResponse, ApiError> {
        let url = format!("{}/from-gan", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Retrieves a gift card using a secure payment token that represents the gift card.
    pub async fn retrieve_gift_card_from_nonce(
        &self,
        body: &RetrieveGiftCardFromNonceRequest,
    ) -> Result<RetrieveGiftCardFromNonceResponse, ApiError> {
        let url = format!("{}/from-nonce", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Links a customer to a gift card, which is also referred to as adding a card on file.
    ///
    /// `gift_card_id`: The ID of the gift card to be linked.
    pub async fn link_customer_to_gift_card(
        &self,
        gift_card_id: &str,
        body: &LinkCustomerToGiftCardRequest,
    ) -> Result<LinkCustomerToGiftCardResponse, ApiError> {
        let url = format!("{}/{}/link-customer", &self.url(), gift_card_id);
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Unlinks a customer from a gift card, which is also referred to as removing a card on file.
    ///
    /// `gift_card_id`: The ID of the gift card to be linked.
    pub async fn unlink_customer_from_gift_card(
        &self,
        gift_card_id: &str,
        body: &UnlinkCustomerFromGiftCardRequest,
    ) -> Result<UnlinkCustomerFromGiftCardResponse, ApiError> {
        let url = format!("{}/{}/unlink-customer", &self.url(), gift_card_id);
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Retrieves a gift card using the gift card ID.
    ///
    /// `id`: The ID of the gift card to retrieve.
    pub async fn retrieve_gift_card(&self, id: &str) -> Result<RetrieveGiftCardResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), id);
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
