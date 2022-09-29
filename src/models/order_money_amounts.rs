//! Model struct for OrderMoneyAmounts type

use serde::{Deserialize, Serialize};

use super::Money;

/// A collection of various money amounts.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderMoneyAmounts {
    /// The total money.
    pub total_money: Option<Money>,
    /// The money associated with taxes.
    pub tax_money: Option<Money>,
    /// The money associated with discounts.
    pub discount_money: Option<Money>,
    /// The money associated with tips.
    pub tip_money: Option<Money>,
    /// The money associated with service charges.
    pub service_charge_money: Option<Money>,
}
