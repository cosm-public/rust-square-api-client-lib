//! Model struct for CatalogQueryExact type.

use serde::Serialize;

/// The query filter to return the search result by exact match of the specified attribute name and
/// value.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CatalogQueryExact {
    /// The name of the attribute to be searched. Matching of the attribute name is exact.
    ///
    /// Min Length 1
    pub attribute_name: String,
    /// The desired value of the search attribute. Matching of the attribute value is case
    /// insensitive and can be partial. For example, if a specified value of "sma", objects with the
    /// named attribute value of "Small", "small" are both matched.
    pub attribute_value: String,
}
