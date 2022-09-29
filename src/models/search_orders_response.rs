//! Model struct for SearchOrdersResponse type

use serde::Deserialize;

use super::{errors::Error, Order, OrderEntry};

/// This is a model struct for SearchOrdersResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct SearchOrdersResponse {
    /// A list of [OrderEntries](OrderEntry) that fit the query conditions. The list is populated
    /// only if `return_entries` is set to `true` in the request.
    pub order_entries: Option<Vec<OrderEntry>>,
    /// A list of [Order] objects that match the query conditions. The list is populated only if
    /// `return_entries` is set to `false` in the request.
    pub orders: Option<Vec<Order>>,
    /// The pagination cursor to be used in a subsequent request. If unset, this is the final
    /// response. For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// [Error]s encountered during the search
    pub errors: Option<Vec<Error>>,
}
