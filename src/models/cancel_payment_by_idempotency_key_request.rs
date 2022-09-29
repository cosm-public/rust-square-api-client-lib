//! Model struct for CancelPaymentByIdempotencyKeyRequest type.

use serde::Serialize;

/// This is a model struct for CancelPaymentByIdempotencyRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CancelPaymentByIdempotencyKeyRequest {
    /// The `idempotency_key` identifying the payment to be canceled.
    pub idempotency_key: String,
}
