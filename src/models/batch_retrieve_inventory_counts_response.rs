//! Model struct for BatchRetrieveInventoryCountsResponse type

use serde::Deserialize;

use super::{errors::Error, InventoryCount};

/// This is a model struct for BatchRetrieveInventoryCountsResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BatchRetrieveInventoryCountsResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The current counts for all objects referenced in the request.
    pub counts: Option<Vec<InventoryCount>>,
    /// The pagination cursor to be used in a subsequent request. If unset, this is the final response.
    pub cursor: Option<String>,
}
