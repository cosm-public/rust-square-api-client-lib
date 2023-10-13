//! Response body struct for Delete Payment Link API

use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for DeleteInvoiceResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct DeletePaymentLinkResponse {
    /// The ID of the link that is deleted.
    pub id: Option<String>,
    /// The ID of the order that is canceled.
    /// When a payment link is deleted, Square updates the the `state` (of the order that the checkout link
    /// created) to CANCELED.
    pub canceled_order_id: Option<String>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
