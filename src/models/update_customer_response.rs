//! Model struct for UpdateCustomerResponse type

use serde::Deserialize;

use super::{errors::Error, Customer};

/// This is a model struct for UpdateCustomerResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct UpdateCustomerResponse {
    /// The updated customer.
    pub customer: Customer,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
