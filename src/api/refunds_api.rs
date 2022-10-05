//! Manage and issue refunds for payments made to Square sellers.
//!
//! The following applies to refunds:
//!
//! * You cannot refund more than what was originally collected.
//! * The refund amount must be available in the account's Square balance. If the amount is not
//!   available, Square attempts to take money out of the associated bank account. Refunds are in a
//!   state of PENDING until the funds are secured.
//! * If funds cannot be secured, the refund is not completed and the buyer does not receive a
//!   credit. The refund has a status of FAILED. Future refunds to this payment are not allowed and
//!   the buyer should be reimbursed by other means.
//! * You can refund only payments with status COMPLETED. You cannot refund an APPROVED payment;
//!   however, you can cancel an approved payment.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::ApiError, GetPaymentRefundResponse, ListPaymentRefundsParameters,
        ListPaymentRefundsResponse, RefundPaymentRequest, RefundPaymentResponse,
    },
};

const DEFAULT_URI: &str = "/refunds";

/// Manage and issue refunds for payments made to Square sellers.
pub struct RefundsApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Refunds API endpoints
    client: HttpClient,
}

impl RefundsApi {
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Retrieves a list of refunds for the account making the request.
    ///
    /// Results are eventually consistent, and new refunds or changes to refunds might take several
    /// seconds to appear.
    ///
    /// The maximum results per page is 100.
    pub async fn list_payment_refunds(
        &self,
        params: &ListPaymentRefundsParameters,
    ) -> Result<ListPaymentRefundsResponse, ApiError> {
        let url = format!("{}{}", &self.url(), params.to_query_string());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Refunds a payment.
    ///
    /// You can refund the entire payment amount or a portion of it. You can use this endpoint to
    /// refund a card payment or record a refund of a cash or external payment. For more
    /// information, see
    /// [Refund Payment](https://developer.squareup.com/docs/payments-api/refund-payments).
    pub async fn refund_payment(
        &self,
        body: &RefundPaymentRequest,
    ) -> Result<RefundPaymentResponse, ApiError> {
        let response = self.client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Retrieves a specific refund using the `refund_id`.
    pub async fn get_payment_refund(
        &self,
        refund_id: &str,
    ) -> Result<GetPaymentRefundResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), refund_id);
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
