//! Model for SearchCatalogItemsRequestStockLevel enum.

use serde::Serialize;

/// Defines supported stock levels of the item inventory.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum SearchCatalogItemsRequestStockLevel {
    /// The item inventory is empty.
    Out,
    /// The item inventory is low.
    Low,
}
