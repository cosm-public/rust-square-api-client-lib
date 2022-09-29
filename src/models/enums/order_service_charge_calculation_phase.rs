//! Model for OrderServiceChargeCalculationPhase enum

use serde::{Deserialize, Serialize};

/// Represents a phase in the process of calculating order totals.
///
/// Service charges are applied after the indicated phase.
///
/// [Read more about how order totals are
/// calculated.](https://developer.squareup.com/docs/orders-api/how-it-works#how-totals-are-calculated)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderServiceChargeCalculationPhase {
    /// The service charge is applied after discounts, but before taxes.
    SubtotalPhase,
    /// The service charge is applied after all discounts and taxes are applied.
    TotalPhase,
}
