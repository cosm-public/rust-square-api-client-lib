//! Model struct for GiftCardActivityBlock type

use serde::{Deserialize, Serialize};

/// Represents details about a `BLOCK` [gift card activity type](GiftCardActivityType).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCardActivityBlock {
    /// The reason the gift card was blocked.
    pub reason: String,
}
