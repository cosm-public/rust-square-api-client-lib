//! Model struct for CatalogQueryItemVariationsForItemOptionValues type.

use serde::Serialize;

/// The query filter to return the item variations containing the specified item option value IDs.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CatalogQueryItemVariationsForItemOptionValues {
    /// A set of `CatalogItemOptionValue` IDs to be used to find associated `CatalogItemVariation`s.
    /// All ItemVariations that contain all of the given Item Option Values (in any order) will be
    /// returned.
    pub item_option_value_ids: Option<Vec<String>>,
}
