//! Model struct for GiftCardActivityUnblock type

use serde::{Deserialize, Serialize};

/// Represents details about a `UNBLOCK` [gift card activity type](GiftCardActivityType).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCardActivityUnblock {
    /// The reason the gift card was unblocked.
    pub reason: String,
}
