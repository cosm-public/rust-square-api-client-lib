//! Response body struct for the Update Customer Group API

use serde::Deserialize;

use super::{errors::Error, CustomerGroup};

/// This is a model struct for UpdateCustomerGroupResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct UpdateCustomerGroupResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The successfully updated customer group.
    pub group: CustomerGroup,
}
