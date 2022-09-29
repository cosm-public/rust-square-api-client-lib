//! Model for OrderLineItemDiscountScope enum

use serde::{Deserialize, Serialize};

/// Indicates whether this is a line-item or order-level discount.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLineItemDiscountScope {
    /// Used for reporting only. The original transaction discount scope is currently not supported
    /// by the API.
    OtherDiscountScope,
    /// The discount should be applied to only line items specified by OrderLineItemAppliedDiscount
    /// reference records.
    LineItem,
    /// The discount should be applied to the entire order.
    Order,
}
