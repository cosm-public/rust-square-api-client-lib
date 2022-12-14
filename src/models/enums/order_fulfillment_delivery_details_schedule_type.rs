//! Model for OrderFulfillmentDeliveryDetailsScheduleType enum

use serde::{Deserialize, Serialize};

/// Indicates the schedule type of the delivery fulfillment. If SCHEDULED, then deliver_at is required.
/// If ASAP, then prep_time_duration is required. The default is SCHEDULED.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderFulfillmentDeliveryDetailsScheduleType {
    /// Indicates the fulfillment to deliver at a scheduled deliver time.
    Scheduled,
    /// Indicates that the fulfillment to deliver as soon as possible and should be prepared immediately.
    Asap,
}
