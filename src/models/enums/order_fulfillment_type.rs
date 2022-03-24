//! Model for OrderFulfillmentType enum

use serde::{Deserialize, Serialize};

/// The type of fulfillment.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderFulfillmentType {
    /// A fulfillment to be picked up from a physical [Location] by a recipient.
    Pickup,
    /// A fulfillment to be shipped by a shipping carrier.
    Shipment,
}
