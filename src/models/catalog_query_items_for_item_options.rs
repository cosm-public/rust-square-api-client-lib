//! Model struct for CatalogQueryItemsForItemOptions type.

use serde::Serialize;

/// The query filter to return the items containing the specified item option IDs.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CatalogQueryItemsForItemOptions {
    /// A set of `CatalogItemOption` IDs to be used to find associated `CatalogItem`s. All Items
    /// that contain all of the given Item Options (in any order) will be returned.
    pub item_option_ids: Vec<String>,
}
