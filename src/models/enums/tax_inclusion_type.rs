//! Model for TaxInclusionType enum.

use serde::{Deserialize, Serialize};

/// Whether to the tax amount should be additional to or included in the CatalogItem price.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxInclusionType {
    /// The tax is an additive tax. The tax amount is added on top of the CatalogItemVariation
    /// price. For example, a $1.00 item with a 10% additive tax would have a total cost to the
    /// buyer of $1.10.
    Additive,
    /// The tax is an inclusive tax. The tax amount is included in the CatalogItemVariation price.
    /// For example, a $1.00 item with a 10% inclusive tax would have a total cost to the buyer of
    /// $1.00, with $0.91 (91 cents) of that total being the cost of the item and $0.09 (9 cents)
    /// being tax.
    Inclusive,
}
