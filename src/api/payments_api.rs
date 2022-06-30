//! The Payments API lets developers take and manage payments.
//!
//! Applications need the following input to take a payment:
//!
//! * The amount to charge.
//!
//! * The payment recipient. The payment goes to the account identified by the Authorization header
//! in the API request.
//!
//! * The payment source. The source can be a payment token or card on file.
//!
//! You can generate a payment token using the Web Payments SDK and the In-App Payments SDK. For
//! working code examples, see Square Connect API Examples.
//!
//! A card on file is a credit card, debit card, or gift card that is associated with a customer.
//! You can create a customer and add a card on file using Square APIs, the Square Seller Dashboard, or the Square Point of Sale application.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::ApiError, CancelPaymentByIdempotencyKeyRequest,
        CancelPaymentByIdempotencyKeyResponse, CancelPaymentResponse, CompletePaymentRequest,
        CompletePaymentResponse, CreatePaymentRequest, CreatePaymentResponse, GetPaymentResponse,
        ListPaymentsParameters, ListPaymentsResponse, UpdatePaymentRequest, UpdatePaymentResponse,
    },
};

const DEFAULT_URI: &str = "/payments";

/// The Payments API lets developers take and manage payments.
pub struct PaymentsApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Payments API endpoints
    client: HttpClient,
}

impl PaymentsApi {
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Cancels (voids) a payment.
    ///
    /// You can use this endpoint to cancel a payment with the APPROVED `status`.
    pub async fn cancel_payment(
        &self,
        payment_id: &str,
    ) -> Result<CancelPaymentResponse, ApiError> {
        let url = format!("{}/{}/cancel", &self.url(), payment_id);
        let response = self.client.post::<Option<()>>(&url, &None).await?;

        response.deserialize().await
    }

    /// Cancels (voids) a payment identified by the idempotency key that is specified in the
    /// request.
    ///
    /// Use this method when the status of a `CreatePayment` request is unknown (for example, after
    /// you send a `CreatePayment` request, a network error occurs and you do not get a response).
    /// In this case, you can direct Square to cancel the payment using this endpoint. In the
    /// request, you provide the same idempotency key that you provided in your `CreatePayment`
    /// request that you want to cancel. After canceling the payment, you can submit your
    /// `CreatePayment` request again.
    ///
    /// Note that if no payment with the specified idempotency key is found, no action is taken and
    /// the endpoint returns successfully.
    pub async fn cancel_payment_by_idempotency_key(
        &self,
        body: &CancelPaymentByIdempotencyKeyRequest,
    ) -> Result<CancelPaymentByIdempotencyKeyResponse, ApiError> {
        let url = format!("{}/cancel", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Completes (captures) a payment.
    ///
    /// By default, payments are set to complete immediately after they are created.
    ///
    /// You can use this endpoint to complete a payment with the APPROVED `status`.
    pub async fn complete_payment(
        &self,
        payment_id: &str,
        body: &CompletePaymentRequest,
    ) -> Result<CompletePaymentResponse, ApiError> {
        let url = format!("{}/{}/complete", &self.url(), payment_id);
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Creates a payment using the provided source.
    ///
    /// You can use this endpoint to charge a card (credit/debit card or Square gift card) or record
    /// a payment that the seller received outside of Square (cash payment from a buyer or a payment
    /// that an external entity processed on behalf of the seller).
    ///
    /// The endpoint creates a [Payment] object and returns it in the response.
    pub async fn create_payment(
        &self,
        body: &CreatePaymentRequest,
    ) -> Result<CreatePaymentResponse, ApiError> {
        let response = self.client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Retrieves details for a specific payment
    pub async fn get_payment(&self, payment_id: &str) -> Result<GetPaymentResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), payment_id);
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Retrieves a list of payments taken by the account making the request.
    ///
    /// Results are eventually consistent, and new payments or changes to payments might take
    /// several seconds to appear.
    ///
    /// The maximum results per page is 100.
    pub async fn list_payments(
        &self,
        params: &ListPaymentsParameters,
    ) -> Result<ListPaymentsResponse, ApiError> {
        let url = format!("{}{}", &self.url(), params.to_query_string());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates a payment with the APPROVED status.
    ///
    /// You can update the `amount_money` and `tip_money` using this endpoint.
    pub async fn update_payment(
        &self,
        payment_id: &str,
        body: &UpdatePaymentRequest,
    ) -> Result<UpdatePaymentResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), payment_id);
        let response = self.client.put(&url, &body).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
