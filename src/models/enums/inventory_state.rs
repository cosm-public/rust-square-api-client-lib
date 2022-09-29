//! Model for InventoryState enum.

use serde::{Deserialize, Serialize};

/// A type of state for the related quantity of items
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InventoryState {
    Custom,
    InStock,
    Sold,
    ReturnedByCustomer,
    ReservedForSale,
    SoldOnline,
    OrderedFromVendor,
    InTransitTo,
    None,
    Waste,
    UnlinkedReturn,
    Composed,
    Decomposed,
    SuportedByNewerVersion,
}
