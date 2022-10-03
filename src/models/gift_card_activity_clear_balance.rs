//! Model struct for GiftCardActivityClearBalance type

use serde::{Deserialize, Serialize};

/// Represents details about a `CLEAR_BALANCE` [gift card activity type](GiftCardActivityType).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCardActivityClearBalance {
    /// The reason the gift card balance was cleared.
    pub reason: String,
}
