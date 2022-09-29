//! Model struct for BatchRetrieveInventoryChangesResponse type

use serde::Deserialize;

use super::{errors::Error, InventoryChange};

/// This is a model struct for BatchRetrieveInventoryChangesResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BatchRetrieveInventoryChangesResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The current counts for all objects referenced in the request.
    pub changes: Option<Vec<InventoryChange>>,
    /// The pagination cursor to be used in a subsequent request. If unset, this is the final response.
    pub cursor: Option<String>,
}
