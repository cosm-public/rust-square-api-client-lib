//! Apple Pay support APIs
//!
//! The Apple Pay APIs provides an easy way for platform developers to bulk activate Web Apple Pay
//! with Square for merchants using their platform.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{errors::ApiError, RegisterDomainRequest, RegisterDomainResponse},
};

const DEFAULT_URI: &str = "/apple-pay/domains";

/// Apple Pay support APIs
pub struct ApplePayApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Apple Pay API endpoints
    client: HttpClient,
}

impl ApplePayApi {
    /// Instantiates a new `ApplePayApi`
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Activates a domain for use with Apple Pay on the Web and Square.
    ///
    /// A validation is performed on this domain by Apple to ensure that it is properly set up as an
    /// Apple Pay enabled domain.
    ///
    /// This endpoint provides an easy way for platform developers to bulk activate Apple Pay on the
    /// Web with Square for merchants using their platform.
    ///
    /// Note: The SqPaymentForm library is deprecated as of May 13, 2021, and will only receive
    /// critical security updates until it is retired on October 31, 2022. You must migrate your
    /// payment form code to the Web Payments SDK to continue using your domain for Apple Pay. For
    /// more information on migrating to the Web Payments SDK, see [Migrate to the Web Payments
    /// SDK](https://developer.squareup.com/docs/web-payments/migrate).
    ///
    /// To learn more about the Web Payments SDK and how to add Apple Pay, see [Take an Apple Pay
    /// Payment](https://developer.squareup.com/docs/web-payments/apple-pay).
    pub async fn register_domain(
        &self,
        body: &RegisterDomainRequest,
    ) -> Result<RegisterDomainResponse, ApiError> {
        let response = self.client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
