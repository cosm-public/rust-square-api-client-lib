//! Model struct for CatalogQueryPrefix type.

use serde::Serialize;

/// The query filter to return the search result whose named attribute values are prefixed by the
/// specified attribute value.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CatalogQueryPrefix {
    /// The name of the attribute to be searched.
    ///
    /// Min Length 1
    pub attribute_name: String,
    /// The desired prefix of the search attribute value.
    ///
    /// Min Length 1
    pub attribute_prefix: String,
}
