//! Model struct for CatalogModifier type.

use serde::{Deserialize, Serialize};

use super::Money;

/// A modifier applicable to items at the time of sale.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogModifier {
    /// The modifier name. This is a searchable attribute for use in applicable query filters, and
    /// its value length is of Unicode code points.
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// The modifier price.
    pub price_money: Option<Money>,
    /// Determines where this `CatalogModifier` appears in the `CatalogModifierList`.
    pub ordinal: Option<i32>,
    /// The ID of the `CatalogModifierList` associated with this modifier.
    pub modifier_list_id: Option<String>,
    /// The IDs of images associated with this `CatalogModifier` instance. Currently these images
    /// are not displayed by Square, but are free to be displayed in 3rd party applications.
    pub image_ids: Option<Vec<String>>,
    /// If `true`, this `CatalogModifier` should be selected by default for this `CatalogItem`.
    pub on_by_default: Option<bool>,
}
