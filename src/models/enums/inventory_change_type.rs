//! Model for InventoryChangeType enum.

use serde::{Deserialize, Serialize};

/// Indicates how the inventory change is applied.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InventoryChangeType {
    /// The change occurred as part of a physical count update.
    PhysicalCount,
    /// The change occurred as part of the normal lifecycle of goods
    /// (e.g., as an inventory adjustment).
    Adjustment,
    /// The change occurred as part of an inventory transfer.
    Transfer,
}
