//! Model struct for InvoiceSort type.

use serde::Serialize;

use super::enums::{InvoiceSortField, SortOrder};

/// Identifies the sort field and sort order.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct InvoiceSort {
    /// The field to use for sorting.
    pub field: InvoiceSortField,
    /// The order to use for sorting the results.
    pub order: Option<SortOrder>,
}
