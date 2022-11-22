//! Model struct for ListSubscriptionEventsParameters (query parameters)

/// This is a model struct for ListSubscriptionEventsParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListSubscriptionEventsParameters {
    /// When the total number of resulting subscription events exceeds the limit of a paged
    /// response, specify the cursor returned from a preceding response here to fetch the next set
    /// of results. If the cursor is unset, the response contains the last page of the results.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// The upper limit on the number of subscription events to return in a paged response.
    pub limit: Option<i32>,
}

impl ListSubscriptionEventsParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListSubscriptionEventsParameters> for String {
    fn from(list_subscription_events_parameters: ListSubscriptionEventsParameters) -> Self {
        list_subscription_events_parameters.to_string()
    }
}

impl ToString for ListSubscriptionEventsParameters {
    fn to_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(limit) = &self.limit {
            params.push(format!("limit={}", limit));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}
