//! Request body struct for the Update Customer Group API

use serde::Serialize;

use super::CustomerGroup;

/// This is a model struct for UpdateCustomerGroupRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateCustomerGroupRequest {
    /// The [CustomerGroup] object including all the updates you want to make.
    pub group: CustomerGroup,
}
