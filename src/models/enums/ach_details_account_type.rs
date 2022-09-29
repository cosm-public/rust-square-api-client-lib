//! Model for AchDetailsAccountType enum.

use serde::{Deserialize, Serialize};

/// A type of bank account performing a transfer for payment.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AchDetailsAccountType {
    Checking,
    Savings,
    Unknown,
}
