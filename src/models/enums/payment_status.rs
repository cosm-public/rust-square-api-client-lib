//! Model for PaymentStatus enum

use serde::{Deserialize, Serialize};

/// The status of a [Payment]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    /// The payment is Approved.
    Approved,
    /// The payment is Pending.
    Pending,
    /// The payment is Completed.
    Completed,
    /// The payment is Canceled.
    Canceled,
    /// The payment is Failed.
    Failed,
}
