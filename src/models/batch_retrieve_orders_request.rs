//! Model struct for BatchRetrieveOrdersRequest type

use serde::Serialize;

/// This is a model class for BatchRetrieveOrdersRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct BatchRetrieveOrdersRequest {
    /// The ID of the location for these orders. This field is optional: omit it to retrieve orders
    /// within the scope of the current authorization's merchant ID.
    pub location_id: Option<String>,
    /// The IDs of the orders to retrieve. A maximum of 100 orders can be retrieved per request.
    pub order_ids: Vec<String>,
}
