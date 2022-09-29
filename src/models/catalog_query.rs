//! Model struct for CatalogQuery type.

use serde::Serialize;

use super::{
    CatalogQueryExact, CatalogQueryItemVariationsForItemOptionValues,
    CatalogQueryItemsForItemOptions, CatalogQueryItemsForModifierList, CatalogQueryItemsForTax,
    CatalogQueryPrefix, CatalogQueryRange, CatalogQuerySet, CatalogQuerySortedAttribute,
    CatalogQueryText,
};

/// A query composed of one or more different types of filters to narrow the scope of targeted
/// objects when calling the `SearchCatalogObjects` endpoint.
///
/// Although a query can have multiple filters, only certain query types can be combined per call to
/// `SearchCatalogObjects`. Any combination of the following types may be used together:
///
/// * [exact_query](CatalogQueryExact)
/// * [prefix_query](CatalogQueryPrefix)
/// * [range_query](CatalogQueryRange)
/// * [sorted_attribute_query](CatalogQuerySortedAttribute)
/// * [text_query](CatalogQueryText) All other query types cannot be combined with any others.
///
/// When a query filter is based on an attribute, the attribute must be searchable. Searchable
/// attributes are listed as follows, along their parent types that can be searched for with
/// applicable query filters.
///
/// * Searchable attribute and objects queryable by searchable attributes **
/// * `name`: `CatalogItem`, `CatalogItemVariation`, `CatalogCategory`, `CatalogTax`,
///   `CatalogDiscount`, `CatalogModifier`, `CatalogModifierList`, `CatalogItemOption`,
///   `CatalogItemOptionValue`
/// * `description`: `CatalogItem`, `CatalogItemOptionValue`
/// * `abbreviation`: `CatalogItem`
/// * `upc`: `CatalogItemVariation`
/// * `sku`: `CatalogItemVariation`
/// * `caption`: `CatalogImage`
/// * `display_name`: `CatalogItemOption`
///
/// For example, to search for [CatalogItem] objects by searchable attributes, you can use the
/// `"name"`, `"description"`, or `"abbreviation"` attribute in an applicable query filter.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CatalogQuery {
    /// A query expression to sort returned query result by the given attribute.
    pub sorted_attribute_query: Option<CatalogQuerySortedAttribute>,
    /// An exact query expression to return objects with attribute name and value matching the
    /// specified attribute name and value exactly. Value matching is case insensitive.
    pub exact_query: Option<CatalogQueryExact>,
    /// A set query expression to return objects with attribute name and value matching the
    /// specified attribute name and any of the specified attribute values exactly. Value matching
    /// is case insensitive.
    pub set_query: Option<CatalogQuerySet>,
    /// A prefix query expression to return objects with attribute values that have a prefix
    /// matching the specified string value. Value matching is case insensitive.
    pub prefix_query: Option<CatalogQueryPrefix>,
    /// A range query expression to return objects with numeric values that lie in the specified
    /// range.
    pub range_query: Option<CatalogQueryRange>,
    /// A text query expression to return objects whose searchable attributes contain all of the
    /// given keywords, irrespective of their order. For example, if a `CatalogItem` contains custom
    /// attribute values of `{"name": "t-shirt"}` and `{"description": "Small, Purple"}`, the query
    /// filter of `{"keywords": ["shirt", "sma", "purp"]}` returns this item.
    pub text_query: Option<CatalogQueryText>,
    /// A query expression to return items that have any of the specified taxes (as identified by
    /// the corresponding `CatalogTax` object IDs) enabled.
    pub items_for_tax_query: Option<CatalogQueryItemsForTax>,
    /// A query expression to return items that have any of the given modifier list (as identified
    /// by the corresponding `CatalogModifierList`s IDs) enabled.
    pub items_for_modifier_list_query: Option<CatalogQueryItemsForModifierList>,
    /// A query expression to return items that contains the specified item options (as identified
    /// the corresponding `CatalogItemOption` IDs).
    pub items_for_item_options_query: Option<CatalogQueryItemsForItemOptions>,
    /// A query expression to return item variations (of the CatalogItemVariation type) that contain
    /// all of the specified `CatalogItemOption` IDs.
    pub item_variations_for_item_option_values_query:
        Option<CatalogQueryItemVariationsForItemOptionValues>,
}
