//! Request body struct for the Create Invoice API

use serde::Serialize;

use super::Invoice;

/// This is a model struct for CreateInvoiceRequest type
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CreateInvoiceRequest {
    /// The invoice to create.
    pub invoice: Invoice,
    /// A unique string that identifies the `CreateInvoice` request. If you do not provide
    /// `idempotency_key` (or provide an empty string as the value), the endpoint treats each
    /// request as independent.
    ///
    /// For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/build-basics/common-api-patterns/idempotency).
    ///
    /// Max Length: 128
    pub idempotency_key: Option<String>,
}
