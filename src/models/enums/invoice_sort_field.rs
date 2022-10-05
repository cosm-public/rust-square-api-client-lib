//! Model for InvoiceSortField enum.

use serde::Serialize;

/// Indicates the status of an invoice.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceSortField {
    /// The field works as follows:
    ///
    /// - If the invoice is a draft, it uses the invoice `created_at` date.
    /// - If the invoice is scheduled for publication, it uses the `scheduled_at` date.
    /// - If the invoice is published, it uses the invoice publication date.
    InvoiceSortDate,
}

impl Default for InvoiceSortField {
    fn default() -> Self {
        Self::InvoiceSortDate
    }
}
