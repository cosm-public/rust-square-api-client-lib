//! Model struct for CatalogQueryRange type.

use serde::Serialize;

/// The query filter to return the search result whose named attribute values fall between the
/// specified range.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CatalogQueryRange {
    /// The name of the attribute to be searched.
    ///
    /// Min Length 1
    pub attribute_name: String,
    /// The desired minimum value for the search attribute (inclusive).
    pub attribute_min_value: Option<i64>,
    /// The desired maximum value for the search attribute (inclusive).
    pub attribute_max_value: Option<i64>,
}
