//! Request struct for the Update Item Modifier Lists API

use serde::Serialize;

/// This is a model struct for UpdateItemModifierListsRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateItemModifierListsRequest {
    /// The IDs of the catalog items associated with the CatalogModifierList objects being updated.
    pub item_ids: Vec<String>,
    /// The IDs of the CatalogModifierList objects to enable for the CatalogItem. At least one of
    /// `modifier_lists_to_enable` or `modifier_lists_to_disable` must be specified.
    pub modifier_lists_to_enable: Option<Vec<String>>,
    /// The IDs of the CatalogModifierList objects to disable for the CatalogItem. At least one of
    /// `modifier_lists_to_enable` or `modifier_lists_to_disable` must be specified.
    pub modifier_lists_to_disable: Option<Vec<String>>,
}
