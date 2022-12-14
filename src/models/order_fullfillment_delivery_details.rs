//! Model struct for OrderFulfillmentShipmentDetails type

use serde::{Deserialize, Serialize};

use super::{
    enums::OrderFulfillmentDeliveryDetailsScheduleType, DateTime, OrderFulfillmentRecipient,
};

/// Describes delivery details of an order fulfillment.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderFulfillmentDeliveryDetails {
    /// Information about the person meant to receive this shipment fulfillment.
    pub recipient: Option<OrderFulfillmentRecipient>,
    /// Indicates the fulfillment delivery schedule type. If SCHEDULED, then deliver_at is required.
    /// If ASAP, then prep_time_duration is required. The default is SCHEDULED
    pub schedule_type: Option<OrderFulfillmentDeliveryDetailsScheduleType>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when the shipment was requested.
    pub placed_at: Option<DateTime>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when fulfillment is supposed to be delivered. Required When the schedule_type is ASAP
    pub deliver_at: Option<DateTime>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    ///  what is the duration of time it takes to prepare and deliver this fulfillment
    pub prep_time_duration: Option<DateTime>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating the time period after the deliver_at timestamp in which to deliver the order.
    pub delivery_window_duration: Option<DateTime>,
    /// Provides additional instructions about the delivery fulfillment.
    /// It is displayed in the Square Point of Sale application and set by the API.
    pub note: Option<String>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when this fulfillment was moved to the COMPLETED state, which indicates that the fulfillment
    /// has been completed by the seller.
    pub completed_at: Option<DateTime>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when this fulfillment was moved to the RESERVED state, which indicates that the fulfillment
    /// has been started by the seller.
    pub in_progress_at: Option<DateTime>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when this fulfillment was moved to the FAILED state, which indicates that the fulfillment
    /// has been rejected.
    pub rejected_at: Option<DateTime>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when this fulfillment was moved to the PREPARED state, which indicates that the fulfillment
    /// has been marked by the seller as ready to be picked up.
    pub ready_at: Option<DateTime>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when this fulfillment has been delivered
    pub delivered_at: Option<DateTime>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating the shipment was canceled.
    pub canceled_at: Option<DateTime>,
    /// A description of why the shipment was canceled.
    pub cancel_reason: Option<String>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when an order can be picked up by the courier for delivery
    pub courier_pickup_at: Option<DateTime>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// the period of time in which the order should be picked up by the courier after the courier_pickup_at
    pub courier_pickup_window_duration: Option<DateTime>,
    /// A description of why the shipment failed to be completed.
    pub is_no_contact_delivery: Option<bool>,
    /// A note to provide additional instructions about how to deliver the order.
    pub dropoff_notes: Option<String>,
    /// The name of the courier provider
    pub courier_provider_name: Option<String>,
    /// The support phone number of the courier.
    pub courier_support_phone_number: Option<String>,
    /// The identifier for the delivery created by Square.
    pub square_delivery_id: Option<String>,
    /// The identifier for the delivery created by the third-party courier service.
    pub external_delivery_id: Option<String>,
    /// The flag to indicate the delivery is managed by a third party (ie DoorDash),
    /// which means we may not receive all recipient information for PII purposes.
    pub managed_delivery: Option<bool>,
}
