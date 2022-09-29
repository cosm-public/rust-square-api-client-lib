//! Model struct for OrderReturnTax type

use serde::{Deserialize, Serialize};

use super::{
    enums::{OrderLineItemTaxScope, OrderLineItemTaxType},
    Money,
};

/// Represents a tax being returned that applies to one or more return line items in an order.
///
/// Fixed-amount, order-scoped taxes are distributed across all non-zero return line item totals.
/// The amount distributed to each return line item is relative to that item’s contribution to the
/// order subtotal.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderReturnTax {
    /// A unique ID that identifies the returned tax only within this order.
    pub uid: Option<String>,
    /// The tax `uid` from the order that contains the original tax charge.
    pub source_tax_id: Option<String>,
    /// The catalog object ID referencing [CatalogTax].
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this tax references.
    pub catalog_version: Option<i64>,
    /// The tax's name.
    pub name: Option<String>,
    /// Indicates the calculation method used to apply the tax.
    pub r#type: Option<OrderLineItemTaxType>,
    /// The percentage of the tax, as a string representation of a decimal number. For example, a
    /// value of "7.25" corresponds to a percentage of 7.25%.
    pub percentage: Option<String>,
    /// The amount of money applied by the tax in an order.
    pub applied_money: Option<Money>,
    /// Indicates the level at which the `OrderReturnTax` applies. For `ORDER` scoped taxes, Square
    /// generates references in `applied_taxes` on all `OrderReturnLineItems`. For `LINE_ITEM`
    /// scoped taxes, the tax is only applied to `OrderReturnLineItems` with references in their
    /// `applied_discounts` field.
    pub scope: Option<OrderLineItemTaxScope>,
}
