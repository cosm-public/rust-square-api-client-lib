//! Model struct for RetrieveCustomerResponse type

use serde::{Deserialize, Serialize};

use super::{errors::Error, Customer};

/// This is a model struct for RetrieveCustomerResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RetrieveCustomerResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The requested customer.
    pub customer: Customer,
}
