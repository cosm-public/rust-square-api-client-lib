//! Response body struct for the Publish Invoice API.

use serde::Deserialize;

use super::{errors::Error, Invoice};

/// This is a model struct for PublishInvoiceResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct PublishInvoiceResponse {
    /// The published invoice.
    pub invoice: Option<Invoice>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
