//! Response struct for the Batch Delete Catalog Objects API

use serde::Deserialize;

use super::{errors::Error, DateTime};

/// This is a model struct for BatchDeleteCatalogObjectsResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BatchDeleteCatalogObjectsResponse {
    /// The IDs of all CatalogObjects deleted by this request.
    pub deleted_object_ids: Option<Vec<String>>,
    /// The database timestamp of this deletion.
    pub deleted_at: Option<DateTime>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
