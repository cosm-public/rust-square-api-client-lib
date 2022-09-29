//! Model struct for SearchOrdersCustomerFilter type

use serde::Serialize;

/// A filter based on the order `customer_id` and any tender `customer_id` associated with the
/// order.
///
/// It does not filter based on the [FulfillmentRecipient](OrderFulfillmentRecipient) `customer_id`.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchOrdersCustomerFilter {
    /// A list of customer IDs to filter by.
    pub customer_ids: Option<Vec<String>>,
}
