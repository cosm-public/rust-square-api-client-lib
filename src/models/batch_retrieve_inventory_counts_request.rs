//! Model struct for BatchRetrieveInventoryCountsRequest type

use serde::{Deserialize, Serialize};

use super::enums::InventoryState;

/// This is a model struct for BatchRetrieveInventoryCountsRequest type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BatchRetrieveInventoryCountsRequest {
    /// The filter to return results by CatalogObject ID. The filter is applicable only when set
    pub catalog_object_ids: Option<Vec<String>>,
    /// The filter to return results by Location ID. This filter is applicable only when set.
    pub location_ids: Option<Vec<String>>,
    /// The filter to return results with their calculated_at value after the given time.
    pub updated_after: String,
    /// A pagination cursor returned by a previous call to this endpoint.
    /// Provide this to retrieve the next set of results for the original query.
    pub cursor: String,
    /// The filter to return results by InventoryState.
    /// The filter is only applicable when set. Ignored are untracked states of NONE, SOLD, and UNLINKED_RETURN.
    /// The default is null.
    pub states: Vec<InventoryState>,
}
