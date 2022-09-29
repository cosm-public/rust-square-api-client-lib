//! Model for CatalogQuickAmountsSettingsOption enum.

use serde::{Deserialize, Serialize};

/// Determines a seller's option on Quick Amounts feature.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogQuickAmountsSettingsOption {
    /// Option for seller to disable Quick Amounts.
    Disabled,
    /// Option for seller to choose manually created Quick Amounts.
    Manual,
    /// Option for seller to choose automatically created Quick Amounts.
    Auto,
}

impl Default for CatalogQuickAmountsSettingsOption {
    fn default() -> Self {
        Self::Disabled
    }
}
