//! Request struct for the Create Catalog Image API

use serde::Serialize;

use super::CatalogObject;

/// This is a model class for CreateCatalogImageRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CreateCatalogImageRequest {
    /// A unique string that identifies this CreateCatalogImage request. Keys can be any valid
    /// string but must be unique for every CreateCatalogImage request.
    ///
    /// See [Idempotency keys](https://developer.squareup.com/docs/basics/api101/idempotency) for
    /// more information.
    pub idempotency_key: String,
    /// Unique ID of the `CatalogObject` to attach this `CatalogImage` object to. Leave this field
    /// empty to create unattached images, for example if you are building an integration where an
    /// image can be attached to catalog items at a later time.
    pub object_id: Option<String>,
    /// The new `CatalogObject` of the `IMAGE` type, namely, a `CatalogImage` object, to encapsulate
    /// the specified image file.
    pub image: CatalogObject,
    /// If this is set to `true`, the image created will be the primary, or first image of the
    /// object referenced by `object_id`. If the `CatalogObject` already has a primary
    /// `CatalogImage`, setting this field to `true` will replace the primary image. If this is set
    /// to `false` and you use the Square API version 2021-12-15 or later, the image id will be
    /// appended to the list of `image_ids` on the object.
    ///
    /// With Square API version 2021-12-15 or later, the default value is `false`. Otherwise, the
    /// effective default value is `true`.
    pub is_primary: Option<bool>,
}
