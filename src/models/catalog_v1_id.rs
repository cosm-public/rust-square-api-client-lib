//! Model struct for CatalogV1Id type.

use serde::{Deserialize, Serialize};

/// A Square API V1 identifier of an item, including the object ID and its associated location ID.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogV1Id {
    /// The ID for an object used in the Square API V1, if the object ID differs from the Square API
    /// V2 object ID.
    pub catalog_v1_id: Option<String>,
    /// The ID of the `Location` this Connect V1 ID is associated with.
    pub location_id: Option<String>,
}
