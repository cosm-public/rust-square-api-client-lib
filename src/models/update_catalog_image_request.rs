//! Request struct for the Update Catalog Image API

use serde::Serialize;

/// This is a model struct for UpdateCatalogImageRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateCatalogImageRequest {
    /// A unique string that identifies this UpdateCatalogImage request. Keys can be any valid
    /// string but must be unique for every UpdateCatalogImage request.
    ///
    /// See [Idempotency keys](https://developer.squareup.com/docs/basics/api101/idempotency) for
    /// more information.
    pub idempotency_key: String,
}
