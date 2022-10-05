//! Request body struct for the Publish Invoice API

use serde::Serialize;

/// This is a model struct for PublishInvoiceRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct PublishInvoiceRequest {
    /// The version of the [Invoice] to publish. This must match the current version of the invoice;
    /// otherwise, the request is rejected.
    pub version: i32,
    /// A unique string that identifies the `PublishInvoice` request. If you do not provide
    /// `idempotency_key` (or provide an empty string as the value), the endpoint treats each
    /// request as independent.
    ///
    /// For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/working-with-apis/idempotency).
    pub idempotency_key: Option<String>,
}
