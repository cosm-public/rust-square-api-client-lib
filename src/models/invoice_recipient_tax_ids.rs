//! Model struct for InvoiceRecipientTaxIds type.

use serde::{Deserialize, Serialize};

/// Represents the tax IDs for an invoice recipient.
///
/// The country of the seller account determines whether the corresponding `tax_ids` field is
/// available for the customer. For more information, see [Invoice recipient tax
/// IDs](https://developer.squareup.com/docs/invoices-api/overview#recipient-tax-ids).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvoiceRecipientTaxIds {
    /// **Read only** The EU VAT identification number for the invoice recipient. For example,
    /// IE3426675K.
    pub eu_vat: Option<String>,
}
