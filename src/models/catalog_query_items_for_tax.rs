//! Model struct for CatalogQueryItemsForTax type.

use serde::Serialize;

/// The query filter to return the items containing the specified tax IDs.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CatalogQueryItemsForTax {
    /// A set of `CatalogTax` IDs to be used to find associated `CatalogItem`s.
    pub tax_ids: Vec<String>,
}
