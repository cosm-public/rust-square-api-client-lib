//! Model for InvoiceCustomFieldPlacement enum.

use serde::{Deserialize, Serialize};

/// Indicates where to render a custom field on the Square-hosted invoice page and in emailed or PDF
/// copies of the invoice.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceCustomFieldPlacement {
    /// Render the custom field above the invoice line items.
    AboveLineItems,
    /// Render the custom field below the invoice line items.
    BelowLineItems,
}

impl Default for InvoiceCustomFieldPlacement {
    fn default() -> Self {
        Self::BelowLineItems
    }
}
