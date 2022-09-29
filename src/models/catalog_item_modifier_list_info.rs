//! Model struct for CatalogItemModifierListInfo type.

use serde::{Deserialize, Serialize};

use super::CatalogModifierOverride;

/// Options to control the properties of a `CatalogModifierList` applied to a `CatalogItem`
/// instance.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogItemModifierListInfo {
    pub modifier_list_id: String,
    pub modifier_overrides: Option<Vec<CatalogModifierOverride>>,
    /// If 0 or larger, the smallest number of `CatalogModifier`s that must be selected from this
    /// `CatalogModifierList`.
    pub min_selected_modifiers: Option<i32>,
    /// If 0 or larger, the largest number of `CatalogModifier`s that can be selected from this
    /// `CatalogModifierList`.
    pub max_selected_modifiers: Option<i32>,
    /// If `true`, enable this `CatalogModifierList`. The default value is `true`.
    pub enabled: Option<bool>,
}
