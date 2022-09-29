//! Model struct for SearchCustomerCreationSourceFilter type

use crate::models::enums::{CustomerCreationSource, CustomerCreationSourceRule};
use serde::Serialize;

/// Filter based on [order fulfillment](OrderFulfillment) information
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchCustomerCreationSourceFilter {
    /// A list of [fulfillment types](OrderFulfillmentType) to filter for. The list returns orders
    /// if any of its fulfillments match any of the fulfillment types listed in this field.
    pub rule: Option<CustomerCreationSourceRule>,
    /// A list of [fulfillment states](OrderFulfillmentState) to filter for. The list returns orders
    /// if any of its fulfillments match any of the fulfillment states listed in this field.
    pub values: Option<Vec<CustomerCreationSource>>,
}
