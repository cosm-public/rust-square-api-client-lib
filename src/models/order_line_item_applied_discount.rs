//! Model struct for OrderLineItemAppliedDiscount type

use serde::{Deserialize, Serialize};

use super::Money;

/// Represents an applied portion of a discount to a line item in an order.
///
/// Order scoped discounts have automatically applied discounts present for each line item.
/// Line-item scoped discounts must have applied discounts added manually for any applicable line
/// items. The corresponding applied money is automatically computed based on participating line
/// items.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderLineItemAppliedDiscount {
    /// A unique ID that identifies the applied discount only within this order.
    pub uid: Option<String>,
    /// The `uid` of the discount that the applied discount represents. It must reference a discount
    /// present in the `order.discounts` field.
    ///
    /// This field is immutable. To change which discounts apply to a line item, you must delete the
    /// discount and re-add it as a new `OrderLineItemAppliedDiscount`.
    pub discount_uid: String,
    /// **Read only** The amount of money applied by the discount to the line item.
    pub applied_money: Option<Money>,
}
