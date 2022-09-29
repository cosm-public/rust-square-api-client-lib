//! Model struct for InventoryChange type

use serde::{Deserialize, Serialize};

use super::{
    enums::InventoryChangeType, CatalogMeasurementUnit, InventoryAdjustment,
    InventoryPhysicalCount, InventoryTransfer,
};

/// Changes created for the request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InventoryChange {
    /// Contains details about the inventory adjustment when type is ADJUSTMENT,
    /// and is unset for all other change types.
    pub adjustment: InventoryAdjustment,
    /// Read only The CatalogMeasurementUnit object representing the catalog measurement unit
    pub measurement_unit: CatalogMeasurementUnit,
    /// ID of the CatalogMeasurementUnit object representing the catalog measurement unit
    pub measurement_unit_id: String,
    /// Contains details about the physical count when type is PHYSICAL_COUNT,
    /// and is unset for all other change types.
    pub physical_count: InventoryPhysicalCount,
    /// TContains details about the inventory transfer when type is TRANSFER,
    ///  and is unset for all other change types.
    pub transfer: InventoryTransfer,
    /// Indicates how the inventory change is applied.
    /// See InventoryChangeType for all possible values.
    pub change_type: InventoryChangeType,
}
