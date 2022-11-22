//! Response body struct for the Search Subscripitons API

use serde::Deserialize;

use super::{errors::Error, Subscription};

/// This is a model struct for SearchSubscriptionsResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct SearchSubscriptionsResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The subscriptions matching the specified query expressions.
    pub subscriptions: Option<Vec<Subscription>>,
    /// When the total number of resulting subscription exceeds the limit of a paged response, the
    /// response includes a cursor for you to use in a subsequent request to fetch the next set of
    /// results. If the cursor is unset, the response contains the last page of the results.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination).
    pub cursor: Option<String>,
}
