//! Model struct for InventoryAdjustmentGroup type

use serde::{Deserialize, Serialize};

use super::enums::InventoryState;

/// The current calculated inventory counts for the requested object and locations.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InventoryAdjustmentGroup {
    /// A unique ID generated by Square for the InventoryAdjustment.
    pub id: String,
    /// The current inventory state for the related quantity of items.
    pub from_state: InventoryState,
    /// The current inventory state for the related quantity of items.
    pub to_state: InventoryState,
    /// Read only The inventory adjustment of the composed variation.
    pub root_adjustment_id: String,
}
