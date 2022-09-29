//! Model struct for InventoryCount type

use serde::Deserialize;

use super::enums::InventoryState;

/// The current calculated inventory counts for the requested object and locations.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct InventoryCount {
    /// Read only An RFC 3339-formatted timestamp that indicates when the most recent physical
    /// count or adjustment affecting the estimated count is received.
    pub calculated_at: String,
    /// The Square-generated ID of the CatalogObject being tracked.
    pub catalog_object_id: String,
    /// The type of the CatalogObject being tracked.
    /// The Inventory API supports setting and reading the "catalog_object_type": "ITEM_VARIATION"
    /// In addition, it can also read the "catalog_object_type": "ITEM"
    pub catalog_object_type: String,
    /// Read only Whether the inventory count is for composed variation (TRUE) or not (FALSE).
    pub is_estimated: bool,
    /// The Square-generated ID of the Location where the related quantity of items is being tracked.
    pub location_id: String,
    /// The number of items affected by the estimated count as a decimal string.
    pub quantity: String,
    /// The current inventory state for the related quantity of items.
    pub state: InventoryState,
}
