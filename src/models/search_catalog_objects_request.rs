//! Request struct for the Search Catalog Objects API

use serde::Serialize;

use super::{enums::CatalogObjectType, CatalogQuery, DateTime};

/// This is a model struct for SearchCatalogObjectsRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchCatalogObjectsRequest {
    /// The pagination cursor returned in the previous response. Leave unset for an initial request.
    /// See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for more
    /// information.
    pub cursor: Option<String>,
    /// The desired set of object types to appear in the search results.
    ///
    /// If this is unspecified, the operation returns objects of all the top level types at the
    /// version of the Square API used to make the request. Object types that are nested onto other
    /// object types are not included in the defaults.
    ///
    /// At the current API version the default object types are: ITEM, CATEGORY, TAX, DISCOUNT,
    /// MODIFIER_LIST, DINING_OPTION, TAX_EXEMPTION, SERVICE_CHARGE, PRICING_RULE, PRODUCT_SET,
    /// TIME_PERIOD, MEASUREMENT_UNIT, SUBSCRIPTION_PLAN, ITEM_OPTION, CUSTOM_ATTRIBUTE_DEFINITION,
    /// QUICK_AMOUNT_SETTINGS.
    pub object_types: Option<Vec<CatalogObjectType>>,
    /// If `true`, deleted objects will be included in the results. Deleted objects will have their
    /// `is_deleted` field set to `true`.
    pub include_deleted_objects: Option<bool>,
    /// If `true`, the response will include additional objects that are related to the requested
    /// objects. Related objects are objects that are referenced by object ID by the objects in the
    /// response. This is helpful if the objects are being fetched for immediate display to a user.
    /// This process only goes one level deep. Objects referenced by the related objects will not be
    /// included. For example:
    ///
    /// If the `objects` field of the response contains a CatalogItem, its associated
    /// CatalogCategory objects, CatalogTax objects, CatalogImage objects and CatalogModifierLists
    /// will be returned in the `related_objects` field of the response. If the `objects` field of
    /// the response contains a CatalogItemVariation, its parent CatalogItem will be returned in the
    /// `related_objects` field of the response.
    ///
    /// Default value: `false`
    pub include_related_objects: Option<bool>,
    /// Return objects modified after this timestamp. The timestamp is exclusive - objects with a
    /// timestamp equal to `begin_time` will not be included in the response.
    pub begin_time: Option<DateTime>,
    /// A query to be used to filter or sort the results. If no query is specified, the entire
    /// catalog will be returned.
    pub query: Option<CatalogQuery>,
    /// A limit on the number of results to be returned in a single page. The limit is advisory -
    /// the implementation may return more or fewer results. If the supplied limit is negative,
    /// zero, or is higher than the maximum limit of 1,000, it will be ignored.
    pub limit: Option<i32>,
}
