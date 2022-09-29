//! Request struct for the Batch Retrieve Catalog Objects API

use serde::Serialize;

/// This is a model class for BatchRetrieveCatalogObjectsRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct BatchRetrieveCatalogObjectsRequest {
    /// The IDs of the CatalogObjects to be retrieved.
    pub object_ids: Vec<String>,
    /// If `true`, the response will include additional objects that are related to the requested
    /// objects. Related objects are defined as any objects referenced by ID by the results in the
    /// `objects` field of the response. These objects are put in the `related_objects` field.
    /// Setting this to `true` is helpful when the objects are needed for immediate display to a
    /// user. This process only goes one level deep. Objects referenced by the related objects will
    /// not be included. For example,
    ///
    /// if the `objects` field of the response contains a CatalogItem, its associated
    /// CatalogCategory objects, CatalogTax objects, CatalogImage objects and CatalogModifierLists
    /// will be returned in the `related_objects` field of the response. If the `objects` field of
    /// the response contains a CatalogItemVariation, its parent CatalogItem will be returned in the
    /// `related_objects` field of the response.
    ///
    /// Default value: `false`
    pub include_related_objects: Option<bool>,
}
