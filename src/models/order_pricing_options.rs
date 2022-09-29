//! Model struct for OrderPricingOptions type

use serde::{Deserialize, Serialize};

/// Pricing options for an order.
///
/// The options affect how the order's price is calculated. They can be used, for example, to apply
/// automatic price adjustments that are based on preconfigured [pricing rules](CatalogPricingRule).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderPricingOptions {
    /// The option to determine whether pricing rule-based discounts are automatically applied to an
    /// order.
    pub auto_apply_discounts: Option<bool>,
    /// The option to determine whether rule-based taxes are automatically applied to an order when
    /// the criteria of the corresponding rules are met.
    pub auto_apply_taxes: Option<bool>,
}
