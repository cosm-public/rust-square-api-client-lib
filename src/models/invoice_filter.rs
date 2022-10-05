//! Model struct for InvoiceFilter type.

use serde::Serialize;

/// Describes query filters to apply.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct InvoiceFilter {
    /// Limits the search to the specified locations. A location is required. In the current
    /// implementation, only one location can be specified.
    ///
    /// Min Length: 1, Max Length: 1
    pub location_ids: Vec<String>,
    /// Limits the search to the specified customers, within the specified locations. Specifying a
    /// customer is optional. In the current implementation, a maximum of one customer can be
    /// specified.
    ///
    /// Min Length: 1, Max Length: 1
    pub customer_ids: Option<Vec<String>>,
}
