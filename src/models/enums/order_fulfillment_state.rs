//! Model for OrderFulfillmentState enum

use serde::{Deserialize, Serialize};

/// The current state of this fulfillment.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderFulfillmentState {
    /// Indicates that the fulfillment has been proposed.
    Proposed,
    /// Indicates that the fulfillment has been reserved.
    Reserved,
    /// Indicates that the fulfillment has been prepared.
    Prepared,
    /// Indicates that the fulfillment was successfully completed.
    Completed,
    /// Indicates that the fulfillment was canceled.
    Canceled,
    /// Indicates that the fulfillment failed to be completed, but was not explicitly canceled.
    Failed,
}
