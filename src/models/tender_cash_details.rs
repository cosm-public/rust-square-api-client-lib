//! Model struct for TenderCashDetails type

use serde::Deserialize;

use super::Money;

/// This is a model struct for TenderCashDetails type.
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq)]
pub struct TenderCashDetails {
    /// Represents an amount of money. `Money` fields can be signed or unsigned. Fields that do not
    /// explicitly define whether they are signed or unsigned are considered unsigned and can only
    /// hold positive amounts. For signed fields, the sign of the value indicates the purpose of the
    /// money transfer. See [Working with Monetary
    /// Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts) for
    /// more information.
    pub buyer_tendered_money: Money,
    /// Represents an amount of money. `Money` fields can be signed or unsigned. Fields that do not
    /// explicitly define whether they are signed or unsigned are considered unsigned and can only
    /// hold positive amounts. For signed fields, the sign of the value indicates the purpose of the
    /// money transfer. See [Working with Monetary
    /// Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts) for
    /// more information.
    pub change_back_money: Money,
}
