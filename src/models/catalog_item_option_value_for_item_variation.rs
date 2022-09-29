//! Model struct for CatalogItemOptionValueForItemVariation type.

use serde::{Deserialize, Serialize};

/// A `CatalogItemOptionValue` links an item variation to an item option as an item option value.
///
/// For example, a t-shirt item may offer a color option and a size option. An item option value
/// would represent each variation of t-shirt: For example, "Color:Red, Size:Small" or
/// "Color:Blue, Size:Medium".
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogItemOptionValueForItemVariation {
    /// The unique id of an item option.
    pub item_option_id: Option<String>,
    /// The unique id of the selected value for the item option.
    pub item_option_value_id: Option<String>,
}
