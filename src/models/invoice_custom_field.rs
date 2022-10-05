//! Model struct for InvoiceCustomField type.

use serde::{Deserialize, Serialize};

use super::enums::InvoiceCustomFieldPlacement;

/// An additional seller-defined and customer-facing field to include on the invoice.
///
/// For more information, see [Custom
/// fields](https://developer.squareup.com/docs/invoices-api/overview#custom-fields).
///
/// Adding custom fields to an invoice requires an [Invoices Plus
/// subscription](https://developer.squareup.com/docs/invoices-api/overview#invoices-plus-subscription).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvoiceCustomField {
    pub label: String,
    pub placement: InvoiceCustomFieldPlacement,
    pub value: Option<String>,
}
