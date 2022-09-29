//! Model struct for CatalogItemOptionValue type.

use serde::{Deserialize, Serialize};

/// An enumerated value that can link a `CatalogItemVariation` to an item option as one of its item
/// option values.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogItemOptionValue {
    /// Unique ID of the associated item option.
    pub item_option_id: Option<String>,
    /// Name of this item option value. This is a searchable attribute for use in applicable query
    /// filters.
    pub name: Option<String>,
    /// A human-readable description for the option value. This is a searchable attribute for use in
    /// applicable query filters.
    pub description: Option<String>,
    /// The HTML-supported hex color for the item option (e.g., "#ff8d4e85"). Only displayed if
    /// `show_colors` is enabled on the parent `ItemOption`. When left unset, `color` defaults to
    /// white ("#ffffff") when `show_colors` is enabled on the parent `ItemOption`.
    pub color: Option<String>,
    /// Determines where this option value appears in a list of option values.
    pub ordinal: Option<i32>,
}
