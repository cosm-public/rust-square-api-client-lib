//! Model struct for CashPaymentDetails type.

use serde::{Deserialize, Serialize};

use super::Money;

/// Stores details about a cash payment.
///
/// Contains only non-confidential information. For more information, see [Take Cash
/// Payments](https://developer.squareup.com/docs/payments-api/take-payments/cash-payments).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CashPaymentDetails {
    /// The amount and currency of the money supplied by the buyer.
    pub buyer_supplied_money: Option<Money>,
    /// The amount of change due back to the buyer. This read-only field is calculated from the
    /// `amount_money` and `buyer_supplied_money` fields.
    pub change_back_money: Option<Money>,
}
