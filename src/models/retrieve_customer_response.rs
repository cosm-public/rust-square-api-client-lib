//! Model struct for RetrieveCardResponse type

use serde::{Deserialize, Serialize};

use super::{errors::Error, Customer};

/// This is a model struct for RetrieveCardResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RetrieveCustomerResponse {
    /// Information on errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// Represents the payment details of a card to be used for payments. These details are
    /// determined by the payment token generated by Web Payments SDK.
    pub customer: Customer,
}
