//! Response struct for the List Invoices API

use serde::Deserialize;

use super::{errors::Error, Invoice};

/// This is a model struct for ListInvoicesResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListInvoicesResponse {
    /// The invoices retrieved.
    pub gift_cards: Option<Vec<Invoice>>,
    /// When a response is truncated, it includes a cursor that you can use in a subsequent request
    /// to retrieve the next set of invoices. If empty, this is the final response. For more
    /// information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
