//! Model struct for CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection type.

use serde::{Deserialize, Serialize};

/// A named selection for this `SELECTION`-type custom attribute definition.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection {
    /// Selection name, unique within `allowed_selections`.
    pub name: String,
    /// Unique ID set by Square.
    pub uid: Option<String>,
}
