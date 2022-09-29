//! Model struct for CompletePaymentRequest type.

use serde::Serialize;

/// This is a model struct for CompletePaymentRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CompletePaymentRequest {
    /// Used for optimistic concurrency. This opaque token identifies the current [Payment] version
    /// that the caller expects. If the server has a different version of the Payment, the update
    /// fails and a response with a VERSION_MISMATCH error is returned.
    pub version_token: Option<String>,
}
