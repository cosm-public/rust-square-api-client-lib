//! Response struct for the Retrieve Inventory Count  API

use serde::Deserialize;

use super::{errors::Error, InventoryCount};

/// This is a model struct for RetrieveInventoryCountResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RetrieveInventoryCountResponse {
    /// The current calculated inventory counts for the requested object and locations.
    pub counts: Option<Vec<InventoryCount>>,
    /// The pagination cursor to be used in a subsequent request. If unset, this is the final
    /// response. For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// [Error]s encountered during the search
    pub errors: Option<Vec<Error>>,
}
