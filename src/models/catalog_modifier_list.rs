//! Model struct for CatalogModifierList type.

use serde::{Deserialize, Serialize};

use super::{enums::CatalogModifierListSelectionType, CatalogObject};

///  list of modifiers applicable to items at the time of sale.
///
/// For example, a "Condiments" modifier list applicable to a "Hot Dog" item may contain "Ketchup",
/// "Mustard", and "Relish" modifiers. Use the `selection_type` field to specify whether or not
/// multiple selections from the modifier list are allowed.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogModifierList {
    /// The name for the `CatalogModifierList` instance. This is a searchable attribute for use in
    /// applicable query filters, and its value length is of Unicode code points.
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// Determines where this modifier list appears in a list of `CatalogModifierList` values.
    pub ordinal: Option<i32>,
    /// Indicates whether multiple options from the modifier list can be applied to a single
    /// `CatalogItem`.
    pub selection_type: Option<CatalogModifierListSelectionType>,
    /// The options included in the `CatalogModifierList`. You must include at least one
    /// `CatalogModifier`. Each CatalogObject must have type `MODIFIER` and contain
    /// `CatalogModifier` data.
    pub modifiers: Option<Vec<CatalogObject>>,
    /// The IDs of images associated with this `CatalogModifierList` instance. Currently these
    /// images are not displayed by Square, but are free to be displayed in 3rd party applications.
    pub image_ids: Option<Vec<String>>,
}
