//! Model struct for OrderReturnLineItemModifier type

use serde::{Deserialize, Serialize};

use super::Money;

/// A line item modifier being returned.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderReturnLineItemModifier {
    /// A unique ID that identifies the return modifier only within this order.
    pub uid: Option<String>,
    /// The modifier `uid` from the order's line item that contains the original sale of this line
    /// item modifier.
    pub source_modifier_uid: Option<String>,
    /// The catalog object ID referencing [CatalogModifier].
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this line item modifier references.
    pub catalog_version: Option<i64>,
    /// The name of the item modifier.
    pub name: Option<String>,
    /// The base price for the modifier.
    ///
    /// `base_price_money` is required for ad hoc modifiers. If both `catalog_object_id` and
    /// `base_price_money` are set, `base_price_money` overrides the predefined [CatalogModifier]
    /// price.
    pub base_price_money: Option<Money>,
    /// **Read only** The total price of the item modifier for its line item. This is the modifier's
    /// `base_price_money` multiplied by the line item's quantity.
    pub total_price_money: Option<Money>,
}
