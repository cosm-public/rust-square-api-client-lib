//! Response body struct for the List Subscription Events API

use serde::Deserialize;

use super::{errors::Error, SubscriptionEvent};

/// This is a model struct for ListSubscriptionEventsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListSubscriptionEventsResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The retrieved subscription events.
    pub subscription_events: Option<Vec<SubscriptionEvent>>,
    /// When the total number of resulting subscription events exceeds the limit of a paged
    /// response, the response includes a cursor for you to use in a subsequent request to fetch the
    /// next set of events. If the cursor is unset, the response contains the last page of the
    /// results.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination).
    pub cursor: Option<String>,
}
