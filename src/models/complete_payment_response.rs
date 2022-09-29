//! Model struct for CompletePaymentResponse type.

use serde::Deserialize;

use super::{errors::Error, Payment};

/// This is a model struct for CompletePaymentResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CompletePaymentResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The successfully completed payment.
    pub payment: Option<Payment>,
}
