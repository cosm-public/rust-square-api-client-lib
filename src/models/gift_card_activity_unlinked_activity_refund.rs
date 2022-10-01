//! Model struct for GiftCardActivityUnlinkedActivityRefund type

use serde::{Deserialize, Serialize};

use super::Money;

/// Represents details about a `UNLINKED_ACTIVITY_REFUND`
/// [gift card activity type](GiftCardActivityType).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCardActivityUnlinkedActivityRefund {
    /// The amount added to the gift card for the refund. This value is a positive integer.
    pub amount_money: Money,
    /// **Read only** The ID of the refunded payment. This field is not used starting in Square
    /// version 2022-06-16.
    pub payment_id: Option<String>,
    /// A client-specified ID that associates the gift card activity with an entity in another
    /// system.
    pub reference_id: Option<String>,
}
