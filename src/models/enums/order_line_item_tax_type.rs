//! Model for OrderLineItemTaxType enum

use serde::{Deserialize, Serialize};

/// Indicates how the tax is applied to the associated line item or order.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLineItemTaxType {
    /// Used for reporting only. The original transaction tax type is currently not supported by the
    /// API.
    UnknownTax,
    /// The tax is an additive tax. The tax amount is added on top of the price. For example, an
    /// item with a cost of 1.00 USD and a 10% additive tax has a total cost to the buyer of 1.10
    /// USD.
    Additive,
    /// The tax is an inclusive tax. Inclusive taxes are already included in the line item price or
    /// order total. For example, an item with a cost of 1.00 USD and a 10% inclusive tax has a
    /// pretax cost of 0.91 USD (91 cents) and a 0.09 (9 cents) tax for a total cost of 1.00 USD to
    /// the buyer.
    Inclusive,
}
