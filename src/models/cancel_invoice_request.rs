//! Request body struct for the Cancel Invoice API

use serde::Serialize;

/// This is a model struct for CancelInvoiceRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CancelInvoiceRequest {
    /// The version of the [Invoice] to cancel. If you do not know the version, you can call
    /// [GetInvoice](https://developer.squareup.com/reference/square/invoices-api/get-invoice) or
    /// [ListInvoices](https://developer.squareup.com/reference/square/invoices-api/list-invoices).
    pub version: i32,
}
