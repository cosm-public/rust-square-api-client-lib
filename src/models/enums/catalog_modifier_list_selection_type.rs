//! Model for CatalogModifierListSelectionType enum.

use serde::{Deserialize, Serialize};

/// Indicates whether a CatalogModifierList supports multiple selections.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogModifierListSelectionType {
    /// Indicates that a CatalogModifierList allows only a single CatalogModifier to be selected.
    Single,
    /// Indicates that a CatalogModifierList allows multiple CatalogModifier to be selected.
    Multiple,
}
