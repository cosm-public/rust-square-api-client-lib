//! Model struct for PayOrderResponse type

use serde::Deserialize;

use super::{errors::Error, Order};

/// This is a model struct for PayOrderResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct PayOrderResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The paid, updated [Order]
    pub order: Order,
}
