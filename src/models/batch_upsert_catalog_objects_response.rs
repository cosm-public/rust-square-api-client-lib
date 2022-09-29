//! Response struct for the Batch Upsert Catalog Objects API

use serde::Deserialize;

use super::{errors::Error, CatalogIdMapping, CatalogObject, DateTime};

/// This is a model struct for BatchUpsertCatalogObjectsResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BatchUpsertCatalogObjectsResponse {
    /// The created successfully created CatalogObjects.
    pub objects: Option<Vec<CatalogObject>>,
    /// The database timestamp of this update.
    pub updated_at: Option<DateTime>,
    /// The mapping between client and server IDs for this upsert.
    pub id_mappings: Option<Vec<CatalogIdMapping>>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
