//! Model struct for CatalogItemOption type.

use serde::{Deserialize, Serialize};

use super::CatalogObject;

/// A group of variations for a `CatalogItem`.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogItemOption {
    /// The item option's display name for the seller. Must be unique across all item options. This
    /// is a searchable attribute for use in applicable query filters.
    pub name: Option<String>,
    /// The item option's display name for the customer. This is a searchable attribute for use in
    /// applicable query filters.
    pub display_name: Option<String>,
    /// The item option's human-readable description. Displayed in the Square Point of Sale app for
    /// the seller and in the Online Store or on receipts for the buyer. This is a searchable
    /// attribute for use in applicable query filters.
    pub description: Option<String>,
    /// If true, display colors for entries in `values` when present.
    pub show_colors: Option<bool>,
    /// A list of CatalogObjects containing the `CatalogItemOptionValues` for this item.
    pub values: Option<Vec<CatalogObject>>,
}
