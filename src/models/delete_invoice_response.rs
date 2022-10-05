//! Response body struct for Delete Invoice API

use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for DeleteInvoiceResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct DeleteInvoiceResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
