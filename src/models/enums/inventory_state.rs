//! Model for InventoryState enum.

use serde::{Deserialize, Serialize};

/// A type of state for the related quantity of items
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InventoryState {
    /// The related quantity of items are in a custom state. READ-ONLY: the Inventory API cannot
    /// move quantities to or from this state.
    Custom,
    /// The related quantity of items are on hand and available for sale.
    InStock,
    /// The related quantity of items were sold as part of an itemized transaction. Quantities in
    /// the SOLD state are no longer tracked.
    Sold,
    /// The related quantity of items were returned through the Square Point of Sale application,
    /// but are not yet available for sale. READ-ONLY: the Inventory API cannot move quantities to
    /// or from this state.
    ReturnedByCustomer,
    /// The related quantity of items are on hand, but not currently available for sale. READ-ONLY:
    /// the Inventory API cannot move quantities to or from this state.
    ReservedForSale,
    /// The related quantity of items were sold online. READ-ONLY: the Inventory API cannot move
    /// quantities to or from this state.
    SoldOnline,
    /// The related quantity of items were ordered from a vendor but not yet received. READ-ONLY:
    /// the Inventory API cannot move quantities to or from this state.
    OrderedFromVendor,
    ///The related quantity of items were received from a vendor but are not yet available for sale.
    /// READ-ONLY: the Inventory API cannot move quantities to or from this state.
    ReceivedFromVendor,
    /// The related quantity of items are in transit between locations. READ-ONLY*: the Inventory
    /// API cannot move quantities to or from this state.
    InTransitTo,
    /// A placeholder indicating that the related quantity of items are not currently tracked in
    /// Square. Transferring quantities from the NONE state to a tracked state (e.g., IN_STOCK)
    /// introduces stock into the system.
    None,
    /// The related quantity of items are lost or damaged and cannot be sold.
    Waste,
    /// The related quantity of items were returned but not linked to a previous transaction.
    /// Unlinked returns are not tracked in Square. Transferring a quantity from UNLINKED_RETURN to
    /// a tracked state (e.g., IN_STOCK) introduces new stock into the system.
    UnlinkedReturn,
    /// The related quantity of items that are part of a composition consisting one or more
    /// components.
    Composed,
    /// The related quantity of items that are part of a component.
    Decomposed,
    /// This state is not supported by this version of the Square API. We recommend that you upgrade
    /// the client to use the appropriate version of the Square API supporting this state.
    SuportedByNewerVersion,
}
