//! Model for PaymentRefundStatus enum

use serde::{Deserialize, Serialize};

/// Indicates a refund's current status.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentRefundStatus {
    /// Awaiting approval.
    Pending,
    /// Successfully completed.
    Completed,
    /// The refund was rejected.
    Rejected,
    /// An error occurred.
    Failed,
}
