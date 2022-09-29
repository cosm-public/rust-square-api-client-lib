//! Response struct for the Create Catalog Image API

use serde::Deserialize;

use super::{errors::Error, CatalogObject};

/// This is a model struct for CreateCatalogImageResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CreateCatalogImageResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The newly created `CatalogImage` including a Square-generated URL for the encapsulated image
    /// file.
    pub image: Option<CatalogObject>,
}
