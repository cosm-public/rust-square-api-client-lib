//! Model for CatalogQuickAmountType enum.

use serde::{Deserialize, Serialize};

/// Determines the type of a specific Quick Amount.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogQuickAmountType {
    /// Quick Amount is created manually by the seller.
    QuickAmountTypeManual,
    /// Quick Amount is generated automatically by machine learning algorithms.
    QuickAmountTypeAuto,
}

impl Default for CatalogQuickAmountType {
    fn default() -> Self {
        Self::QuickAmountTypeAuto
    }
}
