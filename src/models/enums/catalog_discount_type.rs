//! Model for CatalogDiscountType enum.

use serde::{Deserialize, Serialize};

/// How to apply a CatalogDiscount to a CatalogItem.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogDiscountType {
    /// Apply the discount as a fixed percentage (e.g., 5%) off the item price.
    FixedPercentage,
    /// Apply the discount as a fixed amount (e.g., $1.00) off the item price.
    FixedAmount,
    /// Apply the discount as a variable percentage off the item price. The percentage will be
    /// specified at the time of sale.
    VariablePercentage,
    /// Apply the discount as a variable amount off the item price. The amount will be specified at
    /// the time of sale.
    VariableAmount,
}
