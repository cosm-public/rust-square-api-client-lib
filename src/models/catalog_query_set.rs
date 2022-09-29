//! Model struct for CatalogQuerySet type.

use serde::Serialize;

/// The query filter to return the search result(s) by exact match of the specified `attribute_name`
/// and any of the `attribute_values`.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CatalogQuerySet {
    /// The name of the attribute to be searched. Matching of the attribute name is exact.
    ///
    /// Min Length 1
    pub attribute_name: String,
    /// The desired values of the search attribute. Matching of the attribute values is exact and
    /// case insensitive. A maximum of 250 values may be searched in a request.
    pub attribute_values: Vec<String>,
}
