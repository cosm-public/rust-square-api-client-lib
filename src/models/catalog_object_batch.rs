//! Model struct for CatalogObjectBatch type.

use serde::Serialize;

use super::CatalogObject;

/// A batch of catalog objects
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CatalogObjectBatch {
    /// A list of CatalogObjects belonging to this batch.
    pub objects: Vec<CatalogObject>,
}
