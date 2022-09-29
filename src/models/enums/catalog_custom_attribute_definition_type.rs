//! Model for CatalogCustomAttributeDefinitionType enum.

use serde::{Deserialize, Serialize};

/// Defines the possible types for a custom attribute.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogCustomAttributeDefinitionType {
    /// A free-form string containing up to 255 characters.
    String,
    /// A `true` or `false` value.
    Boolean,
    /// A decimal string representation of a number. Can support up to 5 digits after the decimal
    /// point.
    Number,
    /// One or more choices from `allowed_selections`.
    Selection,
}

impl Default for CatalogCustomAttributeDefinitionType {
    fn default() -> Self {
        Self::String
    }
}
