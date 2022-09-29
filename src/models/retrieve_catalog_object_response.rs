//! Response struct for Retrieve Catalog Object API

use serde::Deserialize;

use super::{errors::Error, CatalogObject};

/// This is a model struct for RetrieveCatalogObjectResponse type
#[derive(Clone, Debug, Deserialize, Default, Eq, PartialEq)]
pub struct RetrieveCatalogObjectResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The `CatalogObject`s returned.
    pub object: Option<CatalogObject>,
    /// A list of `CatalogObject`s referenced by the object in the `object` field.
    pub related_objects: Option<Vec<CatalogObject>>,
}
