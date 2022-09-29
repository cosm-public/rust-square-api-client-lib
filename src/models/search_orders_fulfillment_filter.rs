//! Model struct for SearchOrdersFulfillmentFilter type

use serde::Serialize;

use super::enums::{OrderFulfillmentState, OrderFulfillmentType};

/// Filter based on [order fulfillment](OrderFulfillment) information
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchOrdersFulfillmentFilter {
    /// A list of [fulfillment types](OrderFulfillmentType) to filter for. The list returns orders
    /// if any of its fulfillments match any of the fulfillment types listed in this field.
    pub fulfillment_types: Option<Vec<OrderFulfillmentType>>,
    /// A list of [fulfillment states](OrderFulfillmentState) to filter for. The list returns orders
    /// if any of its fulfillments match any of the fulfillment states listed in this field.
    pub fulfillment_states: Option<Vec<OrderFulfillmentState>>,
}
