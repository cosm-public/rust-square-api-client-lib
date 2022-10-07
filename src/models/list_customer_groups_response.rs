//! Response struct for the List Customer Groups API

use serde::Deserialize;

use super::{errors::Error, CustomerGroup};

/// This is a model struct for ListCustomerGroupsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListCustomerGroupsResponse {
    /// A list of customer groups belonging to the current seller.
    pub groups: Option<Vec<CustomerGroup>>,
    /// A pagination cursor to retrieve the next set of results for your original query to the
    /// endpoint. This value is present only if the request succeeded and additional results are
    /// available.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
