//! Request struct for the Search Invoices API

use serde::Serialize;

use super::InvoiceQuery;

/// This is a model struct for SearchInvoicesRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchInvoicesRequest {
    /// Describes the query criteria for searching invoices.
    pub query: InvoiceQuery,
    /// The maximum number of invoices to return (200 is the maximum `limit`). If not provided, the
    /// server uses a default limit of 100 invoices.
    pub limit: Option<i32>,
    /// A pagination cursor returned by a previous call to this endpoint. Provide this cursor to
    /// retrieve the next set of results for your original query.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination).
    pub cursor: Option<String>,
}
