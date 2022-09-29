//! Model for RefundStatus enum

use serde::{Deserialize, Serialize};

/// Indicates a refund's current status.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundStatus {
    /// The refund is pending.
    Pending,
    /// The refund has been approved by Square.
    Approved,
    /// The refund has been rejected by Square.
    Rejected,
    /// The refund failed.
    Failed,
}
