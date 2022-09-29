//! Response struct for the List Catalog API

use serde::Deserialize;

use super::{errors::Error, CatalogObject};

/// This is a model struct for ListCatalogResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListCatalogResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The pagination cursor to be used in a subsequent request. If unset, this is the final
    /// response.
    ///
    /// See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for more
    /// information.
    pub cursor: Option<String>,
    /// The CatalogObjects returned.
    pub objects: Option<Vec<CatalogObject>>,
}
