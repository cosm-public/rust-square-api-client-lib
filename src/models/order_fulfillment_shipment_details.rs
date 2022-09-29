//! Model struct for OrderFulfillmentShipmentDetails type

use serde::{Deserialize, Serialize};

use super::{DateTime, OrderFulfillmentRecipient};

/// Contains the details necessary to fulfill a shipment order.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderFulfillmentShipmentDetails {
    /// Information about the person meant to receive this shipment fulfillment.
    pub recipient: Option<OrderFulfillmentRecipient>,
    /// The shipping carrier being used to ship this fulfillment (such as UPS, FedEx, or USPS).
    pub carrier: Option<String>,
    /// A note with additional information for the shipping carrier.
    pub shipping_note: Option<String>,
    /// A description of the type of shipping product purchased from the carrier (such as
    /// First Class, Priority, or Express).
    pub shipping_type: Option<String>,
    /// The reference number provided by the carrier to track the shipment's progress.
    pub tracking_number: Option<String>,
    /// A link to the tracking webpage on the carrier's website.
    pub tracking_url: Option<String>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when the shipment was requested.
    pub placed_at: Option<DateTime>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when this fulfillment was moved to the RESERVED state, which indicates that preparation of
    /// this shipment has begun.
    pub in_progress_at: Option<DateTime>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when this fulfillment was moved to the PREPARED state, which indicates that the fulfillment
    /// is packaged.
    pub packaged_at: Option<DateTime>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating when the shipment is expected to be delivered to the shipping carrier.
    pub expected_shipped_at: Option<DateTime>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when this fulfillment was moved to the COMPLETED state, which indicates that the fulfillment
    /// has been given to the shipping carrier.
    pub shipped_at: Option<DateTime>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating the shipment was canceled.
    pub canceled_at: Option<DateTime>,
    /// A description of why the shipment was canceled.
    pub cancel_reason: Option<String>,
    /// *Read only* The
    /// [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating
    /// when the shipment failed to be completed.
    pub failed_at: Option<DateTime>,
    /// A description of why the shipment failed to be completed.
    pub failure_reason: Option<String>,
}
