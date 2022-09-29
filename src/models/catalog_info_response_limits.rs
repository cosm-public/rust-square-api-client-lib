//! Model struct for CatalogInfoResponseLimits type

use serde::Deserialize;

/// Limits that apply to the Catalog Info API.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CatalogInfoResponseLimits {
    /// The maximum number of objects that may appear within a single batch in a
    /// `/v2/catalog/batch-upsert` request.
    pub batch_upsert_max_objects_per_batch: Option<i32>,
    /// The maximum number of objects that may appear across all batches in a
    /// `/v2/catalog/batch-upsert` request.
    pub batch_upsert_max_total_objects: Option<i32>,
    /// The maximum number of object IDs that may appear in a `/v2/catalog/batch-retrieve` request.
    pub batch_retrieve_max_object_ids: Option<i32>,
    /// The maximum number of results that may be returned in a page of a `/v2/catalog/search`
    /// response.
    pub search_max_page_limit: Option<i32>,
    /// The maximum number of object IDs that may be included in a single
    /// `/v2/catalog/batch-delete` request.
    pub batch_delete_max_object_ids: Option<i32>,
    /// The maximum number of item IDs that may be included in a single
    /// `/v2/catalog/update-item-taxes` request.
    pub update_item_taxes_max_item_ids: Option<i32>,
    /// The maximum number of tax IDs to be enabled that may be included in a single
    /// `/v2/catalog/update-item-taxes` request.
    pub update_item_taxes_max_taxes_to_enable: Option<i32>,
    /// The maximum number of tax IDs to be disabled that may be included in a single
    /// `/v2/catalog/update-item-taxes` request.
    pub update_item_taxes_max_taxes_to_disable: Option<i32>,
    /// The maximum number of item IDs that may be included in a single
    /// `/v2/catalog/update-item-modifier-lists` request.
    pub update_item_modifier_lists_max_item_ids: Option<i32>,
    /// The maximum number of modifier list IDs to be enabled that may be included in a single
    /// `/v2/catalog/update-item-modifier-lists` request.
    pub update_item_modifier_lists_max_modifier_lists_to_enable: Option<i32>,
    /// The maximum number of modifier list IDs to be disabled that may be included in a single
    /// `/v2/catalog/update-item-modifier-lists` request.
    pub update_item_modifier_lists_max_modifier_lists_to_disable: Option<i32>,
}
