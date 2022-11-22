//! Request body struct for the Search Subscriptions API

use serde::Serialize;

use super::SearchSubscriptionsQuery;

/// This is a model struct for SearchSubscriptionsRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchSubscriptionsRequest {
    /// When the total number of resulting subscriptions exceeds the limit of a paged response,
    /// specify the cursor returned from a preceding response here to fetch the next set of results.
    /// If the cursor is unset, the response contains the last page of the results.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination).
    pub cursor: Option<String>,
    /// The upper limit on the number of subscriptions to return in a paged response.
    ///
    /// Min: 1
    pub limit: Option<i32>,
    /// A subscription query consisting of specified filtering conditions.
    ///
    /// If this query field is unspecified, the `SearchSubscriptions` call will return all
    /// subscriptions.
    pub query: Option<SearchSubscriptionsQuery>,
    /// An option to include related information in the response.
    ///
    /// The supported values are:
    ///
    /// * `actions`: to include scheduled actions on the targeted subscriptions.
    pub include: Option<Vec<String>>,
}
