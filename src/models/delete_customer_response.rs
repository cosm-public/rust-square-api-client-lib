//! Model struct for DeleteCustomerResponse type

use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for DeleteCustomerResponse type
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct DeleteCustomerResponse {
    /// Information on errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
