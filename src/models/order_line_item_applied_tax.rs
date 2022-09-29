//! Model struct for OrderLineItemAppliedTax type

use serde::{Deserialize, Serialize};

use super::Money;

/// Represents an applied portion of a tax to a line item in an order.
///
/// Order-scoped taxes automatically include the applied taxes in each line item. Line item taxes
/// must be referenced from any applicable line items. The corresponding applied money is
/// automatically computed, based on the set of participating line items.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderLineItemAppliedTax {
    /// A unique ID that identifies the applied tax only within this order.
    pub uid: Option<String>,
    /// The `uid` of the tax for which this applied tax represents. It must reference a tax present
    /// in the `order.taxes` field.
    ///
    /// This field is immutable. To change which taxes apply to a line item, delete and add a new
    /// `OrderLineItemAppliedTax`.
    pub tax_uid: String,
    /// **Read only** The amount of money applied by the tax to the line item.
    pub applied_money: Option<Money>,
}
