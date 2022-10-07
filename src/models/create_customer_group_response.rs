//! Response body struct for the Create Customer Group API

/// This is a model struct for CreateCustomerGroupResponse type
use serde::Deserialize;

use super::{errors::Error, CustomerGroup};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CreateCustomerGroupResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The successfully created customer group.
    pub group: CustomerGroup,
}
