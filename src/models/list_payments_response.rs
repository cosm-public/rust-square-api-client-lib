//! Model struct for ListPaymentsResponse type

use serde::Deserialize;

use super::{errors::Error, Payment};

/// This is a model struct for ListPaymentsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListPaymentsResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The requested list of payments.
    pub payments: Option<Vec<Payment>>,
    /// The pagination cursor to be used in a subsequent request. If empty, this is the final
    /// response.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination)
    pub cursor: Option<String>,
}
