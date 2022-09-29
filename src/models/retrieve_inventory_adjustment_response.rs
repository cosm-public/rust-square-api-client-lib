//! Response struct for the Retrieve Inventory Adjustments API

use serde::Deserialize;

use super::{errors::Error, InventoryAdjustment};

/// This is a model struct for RetrieveInventoryAdjustmentResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RetrieveInventoryAdjustmentResponse {
    /// The created successfully created CatalogObjects.
    pub adjustment: Option<InventoryAdjustment>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
