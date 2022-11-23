//! Model for ChangeTiming enum.

use serde::{Deserialize, Serialize};

/// Supported timings when a pending change, as an action, takes place to a subscription.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChangeTiming {
    /// The action occurs immediately.
    Immediate,
    /// The action occurs at the end of the billing cycle.
    EndOfBillingCycle,
}
