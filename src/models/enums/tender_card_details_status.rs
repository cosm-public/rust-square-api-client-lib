//! Model for TenderCardDetailsStatus enum

use serde::{Deserialize, Serialize};

/// Indicates the card transaction's current status.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderCardDetailsStatus {
    /// The card transaction has been authorized but not yet captured.
    Authorized,
    /// The card transaction was authorized and subsequently captured (i.e., completed).
    Captured,
    /// The card transaction was authorized and subsequently voided (i.e., canceled).
    Voided,
    /// The card transaction failed.
    Failed,
}
