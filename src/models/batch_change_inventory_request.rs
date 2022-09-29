//! Model struct for BatchChangeInventoryRequest type
use serde::Serialize;

use super::InventoryChange;

/// This is a model class for BatchChangeInventoryRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct BatchChangeInventoryRequest {
    /// A client-supplied, universally unique identifier (UUID) for the request.
    pub idempotency_key: String,
    /// The set of physical counts and inventory adjustments to be made.
    /// Changes are applied based on the client-supplied timestamp and may be sent out of order.
    pub changes: Option<Vec<InventoryChange>>,
    /// Indicates whether the current physical count should be ignored if the quantity is unchanged.
    ///  Default: true.
    pub ignore_unchanged_count: bool,
}
