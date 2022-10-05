//! Query parameters for the List Invoices API

/// This is a model struct for ListInvoicesParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListInvoicesParameters {
    /// The ID of the location for which to list invoices.
    pub location_id: String,
    /// A pagination cursor returned by a previous call to this endpoint. Provide this cursor to
    /// retrieve the next set of results for your original query.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// The maximum number of invoices to return (200 is the maximum `limit`). If not provided, the
    /// server uses a default limit of 100 invoices.
    pub limit: Option<i32>,
}

impl ListInvoicesParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListInvoicesParameters> for String {
    fn from(list_invoices_parameters: ListInvoicesParameters) -> Self {
        list_invoices_parameters.to_string()
    }
}

impl ToString for ListInvoicesParameters {
    fn to_string(&self) -> String {
        let mut params = Vec::new();

        if !self.location_id.is_empty() {
            params.push(format!("location_id={}", &self.location_id));
        }

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(limit) = self.limit {
            params.push(format!("limit={}", limit));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}
