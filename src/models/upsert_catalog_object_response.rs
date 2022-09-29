//! Response struct for the Upsert Catalog Object API

use serde::Deserialize;

use super::{errors::Error, CatalogIdMapping, CatalogObject};

/// This is a model struct for UpsertCatalogObjectResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct UpsertCatalogObjectResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The successfully created or updated CatalogObject.
    pub catalog_object: Option<CatalogObject>,
    /// The mapping between client and server IDs for this upsert.
    pub id_mappings: Option<Vec<CatalogIdMapping>>,
}
