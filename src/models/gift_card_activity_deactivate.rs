//! Model struct for GiftCardActivityDeactivate type

use serde::{Deserialize, Serialize};

/// Represents details about a `DEACTIVATE` [gift card activity type](GiftCardActivityType).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCardActivityDeactivate {
    /// The reason the gift card was deactivated.
    pub reason: String,
}
