//! Accept payments through a pre-built, Square-hosted checkout page. No frontend required.
//!
//! With the Square Checkout API, your customers can pay for a purchase through a simple, Square-hosted checkout page. It can be integrated into any payments workflow with minimal coding.
//!
//! You can create and configure your checkout page through a CreatePaymentLink request, specifying the accepted payment methods and checkout options like tipping and custom fields. You can also configure a URL for customers to be redirected to once they complete their purchase.
//!
//! First time Square developers should utilize the payment link endpoints to create, update, retrieve, and list checkout pages.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{errors::ApiError, CreatePaymentLinkRequest, CreatePaymentLinkResponse},
};

const DEFAULT_URI: &str = "/online-checkout";

/// The Checkout API lets developers create and delete Square-hosted checkout links.
pub struct CheckoutApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Payments API endpoints
    client: HttpClient,
}

impl CheckoutApi {
    /// Instantiates a new [`CheckoutApi`]
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Creates a Square-hosted checkout page.
    ///
    /// Applications can share the resulting payment link with their buyer to pay for goods and services.
    pub async fn create_payment_link(
        &self,
        body: &CreatePaymentLinkRequest,
    ) -> Result<CreatePaymentLinkResponse, ApiError> {
        let url = format!("{}/payment-links", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
