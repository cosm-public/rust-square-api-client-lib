//! Model for ExcludeStrategy enum.

use serde::{Deserialize, Serialize};

/// Indicates which products matched by a CatalogPricingRule will be excluded if the pricing rule
/// uses an exclude set.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExcludeStrategy {
    /// The least expensive matched products are excluded from the pricing. If the pricing rule is
    /// set to exclude one product and multiple products in the match set qualify as least
    /// expensive, then one will be excluded at random.
    ///
    /// Excluding the least expensive product gives the best discount value to the buyer.
    LeastExpensive,
    /// The most expensive matched product is excluded from the pricing rule. If multiple products
    /// have the same price and all qualify as least expensive, one will be excluded at random.
    ///
    /// This guarantees that the most expensive product is purchased at full price.
    MostExpensive,
}
