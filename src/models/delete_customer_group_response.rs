//! Response body struct for the Delete Customer Group API

use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for DeleteCustomerGroupResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct DeleteCustomerGroupResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
