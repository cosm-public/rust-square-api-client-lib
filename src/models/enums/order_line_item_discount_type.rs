//! Model for OrderLineItemDiscountType enum

use serde::{Deserialize, Serialize};

/// Indicates how the discount is applied to the associated line item or order.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLineItemDiscountType {
    /// Used for reporting only. The original transaction discount type is currently not supported
    /// by the API.
    UnknownDiscount,
    /// Apply the discount as a fixed percentage (such as 5%) off the item price.
    FixedPercentage,
    /// Apply the discount as a fixed monetary value (such as $1.00) off the item price.
    FixedAmount,
    /// Apply the discount as a variable percentage based on the item price.
    ///
    /// The specific discount percentage of a `VARIABLE_PERCENTAGE` discount is assigned at the time
    /// of the purchase.
    VariablePercentage,
    /// Apply the discount as a variable amount based on the item price.
    ///
    /// The specific discount amount of a `VARIABLE_AMOUNT` discount is assigned at the time of the
    /// purchase.
    VariableAmount,
}
