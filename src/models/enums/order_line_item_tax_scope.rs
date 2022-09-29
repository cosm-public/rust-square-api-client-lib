//! Model for OrderLineItemTaxScope enum

use serde::{Deserialize, Serialize};

/// Indicates whether this is a line-item or order-level tax.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLineItemTaxScope {
    /// Used for reporting only. The original transaction tax scope is currently not supported by
    /// the API.
    OtherTaxScope,
    /// The tax should be applied only to line items specified by the `OrderLineItemAppliedTax`
    /// reference records.
    LineItem,
    /// The tax should be applied to the entire order.
    Order,
}
