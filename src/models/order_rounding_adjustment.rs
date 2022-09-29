//! Model struct for OrderRoundingAdjustment type

use serde::{Deserialize, Serialize};

use super::Money;

/// A rounding adjustment of the money being returned.
///
/// Commonly used to apply cash rounding when the minimum unit of the account is smaller than the
/// lowest physical denomination of the currency.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderRoundingAdjustment {
    /// A unique ID that identifies the rounding adjustment only within this order.
    pub uid: Option<String>,
    /// The name of the rounding adjustment from the original sale order.
    pub name: Option<String>,
    /// The actual rounding adjustment amount.
    pub amount_money: Option<Money>,
}
