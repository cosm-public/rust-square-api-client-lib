//! Response struct for the Batch Retrieve Catalog Objects API

use serde::Deserialize;

use super::{errors::Error, CatalogObject};

/// This is a model struct for BatchRetrieveCatalogObjectsResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BatchRetrieveCatalogObjectsResponse {
    /// A list of [CatalogObject]s returned.
    pub objects: Option<Vec<CatalogObject>>,
    /// A list of [CatalogObject]s referenced by the object in the `objects` field.
    pub related_objects: Option<Vec<CatalogObject>>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
