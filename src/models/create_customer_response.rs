//! Model struct for CreateCustomerResponse type

/// This is a model struct for CreateCustomerResponse type

use serde::Deserialize;

use super::Customer;
use super::errors::Error;

#[derive(Debug, Default, Deserialize, Hash, PartialEq)]
pub struct CreateCustomerResponse {
    /// Any errors that occurred during the request.
    pub errors: Vec<Error>,
    /// Represents a Square customer profile in the Customer Directory of a Square seller.
    pub customer: Customer,
}
