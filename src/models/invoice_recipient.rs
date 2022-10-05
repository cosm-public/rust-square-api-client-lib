//! Model struct for InvoiceRecipient type.

use serde::{Deserialize, Serialize};

use super::{Address, InvoiceRecipientTaxIds};

/// Represents a snapshot of customer data.
///
/// This object stores customer data that is displayed on the invoice and that Square uses to
/// deliver the invoice.
///
/// When you provide a customer ID for a draft invoice, Square retrieves the associated customer
/// profile and populates the remaining `InvoiceRecipient` fields. You cannot update these fields
/// after the invoice is published. Square updates the customer ID in response to a merge operation,
/// but does not update other fields.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvoiceRecipient {
    /// The ID of the customer. This is the customer profile ID that you provide when creating a
    /// draft invoice.
    ///
    /// Min Length: 1, Max Length: 255
    pub customer_id: Option<String>,
    /// **Read only** The recipient's given (that is, first) name.
    pub given_name: Option<String>,
    /// **Read only** The recipient's family (that is, last) name.
    pub family_name: Option<String>,
    /// **Read only** The recipient's email address.
    pub email_address: Option<String>,
    /// **Read only** The recipient's physical address.
    pub address: Option<Address>,
    /// **Read only** The recipient's phone number.
    pub phone_number: Option<String>,
    /// **Read only** The name of the recipient's company.
    pub company_name: Option<String>,
    /// **Read only** The recipient's tax IDs. The country of the seller account determines whether
    /// this field is available for the customer. For more information, see [Invoice recipient tax
    /// IDs](https://developer.squareup.com/docs/invoices-api/overview#recipient-tax-ids).
    pub tax_ids: Option<InvoiceRecipientTaxIds>,
}
