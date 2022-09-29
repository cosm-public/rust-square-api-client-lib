//! Model struct for SearchCustomersRequest type

use serde::Serialize;

use super::SearchCustomersQuery;

/// This is a model struct for SearchCustomersRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchCustomersRequest {
    /// A pagination cursor returned by a previous call to this endpoint. Provide this cursor to
    /// retrieve the next set of results for your original query. For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// Query conditions used to filter or sort the results. Note that when retrieving additional
    /// pages using a cursor, you must use the original query.
    pub query: Option<SearchCustomersQuery>,
    /// The maximum number of results to be returned in a single page. It is possible to receive
    /// fewer results than the specified limit on a given page.
    ///
    /// Default: 500
    pub limit: Option<i32>,
}
