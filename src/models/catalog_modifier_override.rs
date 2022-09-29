//! Model struct for CatalogModifierOverride type.

use serde::{Deserialize, Serialize};

/// Options to control how to override the default behavior of the specified modifier.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogModifierOverride {
    /// The ID of the `CatalogModifier` whose default behavior is being overridden.
    pub modifier_id: String,
    /// If `true`, this `CatalogModifier` should be selected by default for this `CatalogItem`.
    pub on_by_default: Option<bool>,
}
