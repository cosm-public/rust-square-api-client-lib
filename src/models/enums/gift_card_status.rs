//! Model for GiftCardStatus enum

use serde::{Deserialize, Serialize};

/// Indicates the gift card state.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardStatus {
    /// The gift card is active and can be used as a payment source.
    Active,
    /// Any activity that changes the gift card balance is permanently forbidden.
    Deactivated,
    /// Any activity that changes the gift card balance is temporarily forbidden.
    Blocked,
    /// The gift card is pending activation. This is the initial state when a gift card is created.
    /// You must activate the gift card before it can be used.
    Pending,
}
