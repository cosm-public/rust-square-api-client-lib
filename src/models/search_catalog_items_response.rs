//! Response struct for the Search Catalog Items API

use serde::Deserialize;

use super::{errors::Error, CatalogObject};

/// This is a model struct for SearchCatalogItemsResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct SearchCatalogItemsResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// Returned items matching the specified query expressions.
    pub items: Option<Vec<CatalogObject>>,
    /// Pagination token used in the next request to return more of the search result.
    pub cursor: Option<String>,
    /// Ids of returned item variations matching the specified query expression.
    pub matched_variation_ids: Option<Vec<String>>,
}
