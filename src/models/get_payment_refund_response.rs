//! Model struct for GetPaymentRefundResponse type

use serde::Deserialize;

use super::{errors::Error, PaymentRefund};

/// This is a model struct for GetPaymentRefundResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct GetPaymentRefundResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The requested `PaymentRefund`.
    pub refund: Option<PaymentRefund>,
}
