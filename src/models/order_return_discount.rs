//! Model struct for OrderReturnDiscount type

use serde::{Deserialize, Serialize};

use super::{
    enums::{OrderLineItemDiscountScope, OrderLineItemDiscountType},
    Money,
};

/// Represents a discount being returned that applies to one or more return line items in an order.
///
/// Fixed-amount, order-scoped discounts are distributed across all non-zero return line item
/// totals. The amount distributed to each return line item is relative to that item’s contribution
/// to the order subtotal.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderReturnDiscount {
    /// A unique ID that identifies the returned discount only within this order.
    pub uid: Option<String>,
    /// The discount `uid` from the order that contains the original application of this discount.
    pub source_discount_uid: Option<String>,
    /// The catalog object ID referencing [CatalogDiscount].
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this discount references.
    pub catalog_version: Option<i64>,
    /// The discount's name.
    pub name: Option<String>,
    /// The type of the discount. If it is created by the API, it is `FIXED_PERCENTAGE` or
    /// `FIXED_AMOUNT`.
    ///
    /// Discounts that do not reference a catalog object ID must have a type of `FIXED_PERCENTAGE`
    /// or `FIXED_AMOUNT`.
    pub r#type: Option<OrderLineItemDiscountType>,
    /// The percentage of the tax, as a string representation of a decimal number. A value of
    /// "7.25" corresponds to a percentage of 7.25%.
    ///
    /// `percentage` is not set for amount-based discounts.
    pub percentage: Option<String>,
    /// The total declared monetary amount of the discount.
    ///
    /// `amount_money` is not set for percentage-based discounts.
    pub amount_money: Option<Money>,
    /// The amount of discount actually applied to this line item. When an amount-based discount is
    /// at the order level, this value is different from `amount_money` because the discount is
    /// distributed across the line items.
    pub applied_money: Option<Money>,
    /// Indicates the level at which the `OrderReturnDiscount` applies. For `ORDER` scoped
    /// discounts, the server generates references in `applied_discounts` on all
    /// `OrderReturnLineItems`. For `LINE_ITEM` scoped discounts, the discount is only applied to
    /// `OrderReturnLineItems` with references in their `applied_discounts` field.
    pub scope: Option<OrderLineItemDiscountScope>,
}
