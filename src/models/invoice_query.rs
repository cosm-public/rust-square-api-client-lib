//! Model struct for InvoiceQuery type.

use serde::Serialize;

use super::{InvoiceFilter, InvoiceSort};

/// Describes query criteria for searching invoices.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct InvoiceQuery {
    /// Query filters to apply in searching invoices. For more information, see [Search for
    /// invoices](https://developer.squareup.com/docs/invoices-api/retrieve-list-search-invoices#search-invoices).
    pub filter: InvoiceFilter,
    /// Describes the sort order for the search result.
    pub sort: Option<InvoiceSort>,
}
