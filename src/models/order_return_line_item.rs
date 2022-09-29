//! Model struct for OrderReturnLineItem type

use serde::{Deserialize, Serialize};

use super::{
    enums::OrderLineItemItemType, Money, OrderLineItemAppliedDiscount, OrderLineItemAppliedTax,
    OrderQuantityUnit, OrderReturnLineItemModifier,
};

/// The line item being returned in an order.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderReturnLineItem {
    /// A unique ID for this return line-item entry.
    pub uid: Option<String>,
    /// The `uid` of the line item in the original sale order.
    pub source_line_item_uid: Option<String>,
    /// The name of the line item.
    pub name: Option<String>,
    /// The quantity returned, formatted as a decimal number. For example, "3".
    ///
    /// Line items with a `quantity_unit` can have non-integer quantities. For example, "1.70000".
    pub quantity: String,
    /// The unit and precision that this return line item's quantity is measured in.
    pub quantity_unit: Option<OrderQuantityUnit>,
    /// The note of the return line item.
    pub note: Option<String>,
    /// The [CatalogItemVariation] ID applied to this return line item.
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this line item references.
    pub catalog_version: Option<i64>,
    /// The name of the variation applied to this return line item.
    pub variation_name: Option<String>,
    /// The type of line item: an itemized return, a non-itemized return (custom amount), or the
    /// return of an unactivated gift card sale.
    pub item_type: Option<OrderLineItemItemType>,
    /// The [CatalogModifier]s applied to this line item.
    pub return_modifiers: Option<Vec<OrderReturnLineItemModifier>>,
    /// The list of references to `OrderReturnTax` entities applied to the return line item. Each
    /// `OrderLineItemAppliedTax` has a `tax_uid` that references the uid of a top-level
    /// `OrderReturnTax` applied to the return line item. On reads, the applied amount is populated.
    pub applied_taxes: Option<Vec<OrderLineItemAppliedTax>>,
    /// The list of references to `OrderReturnDiscount` entities applied to the return line item.
    /// Each `OrderLineItemAppliedDiscount` has a `discount_uid` that references the `uid` of a
    /// top-level `OrderReturnDiscount` applied to the return line item. On reads, the applied
    /// amount is populated.
    pub applied_discounts: Option<Vec<OrderLineItemAppliedDiscount>>,
    /// The base price for a single unit of the line item.
    pub base_price_money: Option<Money>,
    /// Read only The total price of all item variations returned in this line item. The price is
    /// calculated as `base_price_money` multiplied by `quantity` and does not include modifiers.
    pub variation_total_price_money: Option<Money>,
    /// **Read only** The gross return amount of money calculated as (item base price + modifiers
    /// price) * quantity.
    pub gross_return_money: Option<Money>,
    /// **Read only** The total amount of tax money to return for the line item.
    pub total_tax_money: Option<Money>,
    /// **Read only** The total amount of discount money to return for the line item.
    pub total_discount_money: Option<Money>,
    /// **Read only** The total amount of money to return for this line item.
    pub total_money: Option<Money>,
}
