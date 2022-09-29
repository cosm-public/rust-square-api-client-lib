//! Response struct for the Delete Catalog Object API

use serde::Deserialize;

use super::{errors::Error, DateTime};

/// This is a model struct for DeleteCatalogObjectResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct DeleteCatalogObjectResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The IDs of all catalog objects deleted by this request. Multiple IDs may be returned when
    /// associated objects are also deleted, for example a catalog item variation will be deleted
    /// (and its ID included in this field) when its parent catalog item is deleted.
    pub deleted_object_ids: Option<Vec<String>>,
    /// The database timestamp of this deletion.
    pub deleted_at: Option<DateTime>,
}
