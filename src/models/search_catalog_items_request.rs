//! Request struct for the Search Catalog Items API

use serde::Serialize;

use super::enums::{SearchCatalogItemsRequestStockLevel, SortOrder};

/// This is a model struct for SearchCatalogItemsRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchCatalogItemsRequest {
    /// The text filter expression to return items or item variations containing specified text in
    /// the `name`, `description`, or `abbreviation` attribute value of an item, or in the `name`,
    /// `sku`, or `upc` attribute value of an item variation.
    pub text_filter: Option<String>,
    /// The category id query expression to return items containing the specified category IDs.
    pub category_ids: Option<Vec<String>>,
    /// The stock-level query expression to return item variations with the specified stock levels.
    pub stock_levels: Option<Vec<SearchCatalogItemsRequestStockLevel>>,
    /// The enabled-location query expression to return items and item variations having specified
    /// enabled locations.
    pub enabled_location_ids: Option<Vec<String>>,
    /// The pagination token, returned in the previous response, used to fetch the next batch of
    /// pending results.
    pub cursor: Option<String>,
    /// The maximum number of results to return per page. The default value is 100.
    ///
    /// Max 100
    pub limit: Option<i32>,
    /// The order to sort the results by item names. The default sort order is ascending (`ASC`).
    pub sort_order: Option<SortOrder>,
}
