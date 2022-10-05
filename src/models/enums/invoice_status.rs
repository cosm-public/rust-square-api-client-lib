//! Model for InvoiceStatus enum.

use serde::{Deserialize, Serialize};

/// Indicates the status of an invoice.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStatus {
    /// The invoice is a draft. You must publish a draft invoice before Square can process it. A
    /// draft invoice has no `public_url`, so it is not available to customers.
    Draft,
    /// The invoice is published but not yet paid.
    Unpaid,
    /// The invoice is scheduled to be processed. On the scheduled date, Square sends the invoice,
    /// initiates an automatic payment, or takes no action, depending on the delivery method and
    /// payment request settings. Square also sets the invoice status to the appropriate state:
    /// `UNPAID`, `PAID`, `PARTIALLY_PAID`, or `PAYMENT_PENDING`.
    Scheduled,
    /// A partial payment is received for the invoice.
    PartiallyPaid,
    /// The customer paid the invoice in full.
    Paid,
    /// The invoice is paid (or partially paid) and some but not all the amount paid is refunded.
    PartiallyRefunded,
    /// The full amount that the customer paid for the invoice is refunded.
    Refunded,
    /// The invoice is canceled. Square no longer requests payments from the customer. The
    /// `public_url` page remains and is accessible, but it displays the invoice as canceled and
    /// does not accept payment.
    Canceled,
    /// Square canceled the invoice due to suspicious activity.
    Failed,
    /// A payment on the invoice was initiated but has not yet been processed.
    ///
    /// When in this state, invoices cannot be updated and other payments cannot be initiated.
    PaymentPending,
}
