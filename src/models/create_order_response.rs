//! Model struct for CreateOrderResponse type

use serde::Deserialize;

use super::{errors::Error, Order};

/// This is a model struct for CreateOrderResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CreateOrderResponse {
    /// The newly created order.
    pub order: Order,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
