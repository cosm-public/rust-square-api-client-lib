//! Response body struct for the Cancel Invoice API.

use serde::Deserialize;

use super::{errors::Error, Invoice};

/// This is a model struct for CancelInvoiceResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CancelInvoiceResponse {
    /// The canceled invoice.
    pub invoice: Option<Invoice>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
