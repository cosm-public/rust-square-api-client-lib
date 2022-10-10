//! Response struct for the List Customer Segments API

use serde::Deserialize;

use super::{errors::Error, CustomerSegment};

/// This is a model struct for ListCustomerSegmentsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListCustomerSegmentsResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// A list of customer segments belonging to the associated Square account.
    pub segments: Option<Vec<CustomerSegment>>,
    /// A pagination cursor to be used in subsequent calls to `ListCustomerSegments` to retrieve the
    /// next set of query results. The cursor is only present if the request succeeded and
    /// additional results are available.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
}
