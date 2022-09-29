//! Model for CatalogCustomAttributeDefinitionSellerVisibility enum.

use serde::{Deserialize, Serialize};

/// Defines the visibility of a custom attribute to sellers in Square client applications, Square
/// APIs or in Square UIs (including Square Point of Sale applications and Square Dashboard).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogCustomAttributeDefinitionSellerVisibility {
    /// Sellers cannot read this custom attribute in Square client applications or Square APIs.
    SellerVisibilityHidden,
    /// Sellers can read and write this custom attribute value in catalog objects, but cannot edit
    /// the custom attribute definition.
    SellerVisibilityReadWriteValues,
}
