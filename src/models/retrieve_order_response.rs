//! Model struct for RetrieveOrderResponse type

use serde::Deserialize;

use super::{errors::Error, Order};

/// This is a model struct for RetrieveOrderResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RetrieveOrderResponse {
    /// The requested order.
    pub order: Order,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
