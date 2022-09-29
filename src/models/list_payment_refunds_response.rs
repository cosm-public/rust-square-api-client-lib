//! Model struct for ListPaymentRefundsResponse type

use serde::Deserialize;

use super::{errors::Error, PaymentRefund};

/// This is a model struct for ListPaymentRefundsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListPaymentRefundsResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The list of requested refunds.
    pub payments: Option<Vec<PaymentRefund>>,
    /// The pagination cursor to be used in a subsequent request. If empty, this is the final
    /// response.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination)
    pub cursor: Option<String>,
}
