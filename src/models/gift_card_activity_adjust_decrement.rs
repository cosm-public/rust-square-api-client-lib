//! Model struct for GiftCardActivityAdjustDecrement type

use serde::{Deserialize, Serialize};

use super::Money;

/// Represents details about an `ADJUST_DECREMENT` [gift card activity type](GiftCardActivityType).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCardActivityAdjustDecrement {
    /// The amount deducted from the gift card balance. This value is a positive integer.
    pub amount_money: Money,
    /// The reason the gift card balance was adjusted.
    pub reason: String,
}
