//! Request body struct for the Update Invoice API

use serde::Serialize;

use super::Invoice;

/// This is a model struct for UpdateInvoiceRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateInvoiceRequest {
    /// The invoice fields to update. The current invoice version must be specified in the `version`
    /// field. For more information, see [Update an
    /// Invoice](https://developer.squareup.com/docs/invoices-api/update-invoices).
    pub invoice: Invoice,
    /// A unique string that identifies the `UpdateInvoice` request. If you do not provide
    /// `idempotency_key` (or provide an empty string as the value), the endpoint treats each
    /// request as independent.
    ///
    /// For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/basics/api101/idempotency).
    pub idempotency_key: Option<String>,
    /// The list of fields to clear. For examples, see [Update an
    /// Invoice](https://developer.squareup.com/docs/invoices-api/update-invoices).
    pub fields_to_clear: Option<Vec<String>>,
}
