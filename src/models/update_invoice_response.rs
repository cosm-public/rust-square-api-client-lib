//! Response body struct for the Update Invoice API

use serde::Deserialize;

use super::{errors::Error, Invoice};

/// This is a model struct for UpdateInvoiceResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct UpdateInvoiceResponse {
    /// The updated invoice.
    pub invoice: Invoice,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
