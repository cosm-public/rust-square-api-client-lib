//! Model struct for InventoryPhysicalCount type

use serde::{Deserialize, Serialize};

use super::{enums::InventoryState, SourceApplication};

/// Represents the quantity of an item variation that is physically present at a specific location,
/// verified by a seller or a seller's employee.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InventoryPhysicalCount {
    /// A unique Square-generated ID for the InventoryPhysicalCount.
    pub id: String,
    /// The Square-generated ID of the CatalogObject being tracked.
    pub catalog_object_id: String,
    /// The type of the CatalogObject being tracked.
    /// The Inventory API supports setting and reading the "catalog_object_type": "ITEM_VARIATION"
    /// In addition, it can also read the "catalog_object_type": "ITEM"
    pub catalog_object_type: String,
    /// Read only An RFC 3339-formatted timestamp that indicates when the physical count is received.
    pub created_at: String,
    /// The Square-generated ID of the Location where the related quantity of items is being tracked.
    pub location_id: String,
    ///The number of items affected by the estimated count as a decimal string.
    pub quantity: String,
    /// The current inventory state for the related quantity of items.
    pub state: InventoryState,
    /// The Square-generated ID of the Employee responsible for the physical count.
    pub employee_id: String,
    /// A client-generated RFC 3339-formatted timestamp that indicates when the
    /// physical count was examined. For physical count updates, the occurred_at timestamp
    /// cannot be older than 24 hours or in the future relative to the time of the request.
    pub occurred_at: String,
    /// An optional ID provided by the application to tie the InventoryPhysicalCount to an external system.
    pub reference_id: String,
    /// Read only Information about the application with which the physical count is submitted.
    pub source: SourceApplication,
    /// The Square-generated ID of the Team Member responsible for the physical count.
    pub team_member_id: String,
}
