//! Model struct for BatchChangeInventoryResponse type

use serde::Deserialize;

use super::{errors::Error, InventoryChange, InventoryCount};

/// This is a model struct for BatchChangeInventoryResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BatchChangeInventoryResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The current counts for all objects referenced in the request.
    pub counts: Option<Vec<InventoryCount>>,
    /// Changes created for the request.
    pub changes: Option<Vec<InventoryChange>>,
}
