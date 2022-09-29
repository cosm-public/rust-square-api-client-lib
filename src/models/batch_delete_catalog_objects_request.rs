//! Request struct for the Batch Delete Catalog Objects API

use serde::Serialize;

/// This is a model class for BatchDeleteCatalogObjectsRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct BatchDeleteCatalogObjectsRequest {
    pub location_id: Option<String>,
    /// The IDs of the CatalogObjects to be deleted. When an object is deleted, other objects in the
    /// graph that depend on that object will be deleted as well (for example, deleting a
    /// CatalogItem will delete its CatalogItemVariation.
    pub object_ids: Vec<String>,
}
