//! Response struct for the Search Catalog Objects API

use serde::Deserialize;

use super::{errors::Error, CatalogObject};

/// This is a model struct for SearchCatalogObjectsResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct SearchCatalogObjectsResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The pagination cursor to be used in a subsequent request. If unset, this is the final
    /// response. See
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for more information.
    pub cursor: Option<String>,
    /// The CatalogObjects returned.
    pub objects: Option<Vec<CatalogObject>>,
    /// A list of CatalogObjects referenced by the objects in the `objects` field.
    pub related_objects: Option<Vec<CatalogObject>>,
    /// When the associated product catalog was last updated. Will match the value for `end_time` or
    /// `cursor` if either field is included in the `SearchCatalog` request.
    pub latest_time: Option<String>,
}
