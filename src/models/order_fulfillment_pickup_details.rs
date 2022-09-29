//! Model struct for OrderFulfillmentPickupDetails type

use serde::{Deserialize, Serialize};

use super::{
    enums::OrderFulfillmentPickupDetailsScheduleType, DateTime,
    OrderFulfillmentPickupDetailsCurbsidePickupDetails, OrderFulfillmentRecipient,
};

/// This is a model struct for OrderFulfillmentPickupDetails type.
///
/// Contains details necessary to fulfill a pickup order.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderFulfillmentPickupDetails {
    /// Information about the person meant to pick up this fulfillment from a physical location.
    pub recipient: Option<OrderFulfillmentRecipient>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating when this fulfillment expires if it is not accepted. The expiration time can only
    /// be set up to 7 days in the future. If `expires_at` is not set, this pickup fulfillment is
    /// automatically accepted when placed.
    pub expires_at: Option<DateTime>,
    /// The duration of time after which an open and accepted pickup fulfillment is automatically
    /// moved to the `COMPLETED` state. The duration must be in RFC 3339 format (for example,
    /// "P1W3D").
    ///
    /// If not set, this pickup fulfillment remains accepted until it is canceled or completed.
    ///
    /// Example for 2 days, 12 hours, 30 minutes, and 15 seconds: P2DT12H30M15S
    pub auto_complete_duration: Option<String>,
    /// The schedule type of the pickup fulfillment. Defaults to `SCHEDULED`.
    pub schedule_type: Option<OrderFulfillmentPickupDetailsScheduleType>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) that
    /// represents the start of the pickup window.
    ///
    /// For fulfillments with the schedule type `ASAP`, this is automatically set to the current
    /// time plus the expected duration to prepare the fulfillment.
    pub pickup_at: Option<DateTime>,
    /// The window of time in which the order should be picked up after the `pickup_at` timestamp.
    /// Must be in RFC 3339 duration format, e.g., "P1W3D". Can be used as an informational
    /// guideline for merchants.
    ///
    /// Example for 2 days, 12 hours, 30 minutes, and 15 seconds: P2DT12H30M15S
    pub pickup_window_duration: Option<String>,
    /// The duration of time it takes to prepare this fulfillment. The duration must be in RFC 3339
    /// format (for example, "P1W3D").
    ///
    /// Example for 2 days, 12 hours, 30 minutes, and 15 seconds: P2DT12H30M15S
    pub prep_time_duration: Option<String>,
    /// A note meant to provide additional instructions about the pickup fulfillment displayed in
    /// the Square Point of Sale application and set by the API.
    pub note: Option<String>,
    /// Read only
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating when the fulfillment was placed.
    pub placed_at: Option<DateTime>,
    /// Read only
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating when the fulfillment was accepted.
    pub accepted_at: Option<DateTime>,
    /// Read only
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating when the fulfillment was rejected.
    pub rejected_at: Option<DateTime>,
    /// Read only
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating when the fulfillment is marked ready for pickup.
    pub ready_at: Option<DateTime>,
    /// Read only
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating when the fulfillment expired.
    pub expired_at: Option<DateTime>,
    /// Read only
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating when the fulfillment was picked up by the recipient.
    pub picked_up_at: Option<DateTime>,
    /// Read only
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating when the fulfillment was canceled.
    pub canceled_at: Option<DateTime>,
    /// A description of why the pickup was canceled. The maximum length: 100 characters.
    pub cancel_reason: Option<String>,
    /// If set to `true`, indicates that this pickup order is for curbside pickup, not in-store
    /// pickup.
    pub is_curbside_pickup: Option<bool>,
    /// Specific details for curbside pickup. These details can only be populated if
    /// `is_curbside_pickup` is set to `true`.
    pub curbside_pickup_details: Option<OrderFulfillmentPickupDetailsCurbsidePickupDetails>,
}
