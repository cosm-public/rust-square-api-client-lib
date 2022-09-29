//! Model struct for SearchOrdersRequest type

use serde::Serialize;

use super::SearchOrdersQuery;

/// This is a model struct for SearchOrdersRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchOrdersRequest {
    pub location_ids: Option<Vec<String>>,
    /// A pagination cursor returned by a previous call to this endpoint. Provide this cursor to
    /// retrieve the next set of results for your original query. For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// Query conditions used to filter or sort the results. Note that when retrieving additional
    /// pages using a cursor, you must use the original query.
    pub query: Option<SearchOrdersQuery>,
    /// The maximum number of results to be returned in a single page. It is possible to receive
    /// fewer results than the specified limit on a given page.
    ///
    /// Default: 500
    pub limit: Option<i32>,
    /// A Boolean that controls the format of the search results. If true, SearchOrders returns
    /// OrderEntry objects. If false, SearchOrders returns complete order objects.
    pub return_entries: bool,
}
