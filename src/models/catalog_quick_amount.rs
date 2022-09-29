//! Model struct for CatalogQuickAmount type.

use serde::{Deserialize, Serialize};

use super::{enums::CatalogQuickAmountType, Money};

/// Represents a Quick Amount in the Catalog.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogQuickAmount {
    /// Represents the type of the Quick Amount.
    pub r#type: CatalogQuickAmountType,
    /// Represents the actual amount of the Quick Amount with Money type.
    pub amount: Money,
    /// Describes the ranking of the Quick Amount provided by machine learning model, in the range
    /// [0, 100]. MANUAL type amount will always have score = 100.
    pub score: Option<i64>,
    /// The order in which this Quick Amount should be displayed.
    pub ordinal: Option<i64>,
}
