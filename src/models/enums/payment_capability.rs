//! Model for PaymentCapability enum

use serde::{Deserialize, Serialize};

/// Actions that can be performed on a [Payment]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentCapability {
    /// The payment amount can be edited up.
    EditAmountUp,
    /// The payment amount can be edited down.
    EditAmountDown,
    /// The tip amount can be edited up.
    EditTipAmountUp,
    /// The tip amount can be edited down.
    EditTipAmountDown,
}
