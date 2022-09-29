//! Model struct for CatalogIdMapping type.

use serde::Deserialize;

/// A mapping between a temporary client-supplied ID and a permanent server-generated ID.
///
/// When calling UpsertCatalogObject or BatchUpsertCatalogObjects to create a [CatalogObject]
/// instance, you can supply a temporary ID for the to-be-created object, especially when the object
/// is to be referenced elsewhere in the same request body. This temporary ID can be any string
/// unique within the call, but must be prefixed by "#".
///
/// After the request is submitted and the object created, a permanent server-generated ID is
/// assigned to the new object. The permanent ID is unique across the Square catalog.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct CatalogIdMapping {
    /// The client-supplied temporary `#`-prefixed ID for a new `CatalogObject`.
    pub client_object_id: String,
    /// The permanent ID for the CatalogObject created by the server.
    pub object_id: String,
}
