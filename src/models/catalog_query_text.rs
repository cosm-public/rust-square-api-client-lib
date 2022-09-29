//! Model struct for CatalogQueryText type.

use serde::Serialize;

/// The query filter to return the search result whose searchable attribute values contain all of
/// the specified keywords or tokens, independent of the token order or case.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CatalogQueryText {
    /// A list of 1, 2, or 3 search keywords. Keywords with fewer than 3 characters are ignored.
    pub keywords: Vec<String>,
}
