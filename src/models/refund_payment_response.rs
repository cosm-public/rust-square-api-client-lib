//! Model struct for RefundPaymentResponse type

use serde::Deserialize;

use super::{errors::Error, PaymentRefund};

/// This is a model struct for RefundPaymentResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RefundPaymentResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The successfully created `PaymentRefund`.
    pub refund: Option<PaymentRefund>,
}
