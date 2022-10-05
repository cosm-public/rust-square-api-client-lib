//! Response struct for the Search Invoices API

use serde::Deserialize;

use super::{errors::Error, Invoice};

/// This is a model struct for SearchInvoicesResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct SearchInvoicesResponse {
    /// The list of invoices returned by the search.
    pub invoices: Option<Vec<Invoice>>,
    /// When a response is truncated, it includes a cursor that you can use in a subsequent request
    /// to fetch the next set of invoices. If empty, this is the final response.
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination).
    pub cursor: Option<String>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
