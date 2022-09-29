//! Model struct for OrderReturn type

use serde::{Deserialize, Serialize};

use super::{
    OrderMoneyAmounts, OrderReturnDiscount, OrderReturnLineItem, OrderReturnServiceCharge,
    OrderReturnTax, OrderRoundingAdjustment,
};

/// The set of line items, service charges, taxes, discounts, tips, and other items being returned
/// in an order.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderReturn {
    /// A unique ID that identifies the return only within this order.
    pub uid: Option<String>,
    /// An order that contains the original sale of these return line items. This is unset for
    /// unlinked returns.
    pub source_order_id: Option<String>,
    /// A collection of line items that are being returned.
    pub return_line_items: Option<Vec<OrderReturnLineItem>>,
    /// **Read only** A collection of service charges that are being returned.
    pub return_service_charges: Option<Vec<OrderReturnServiceCharge>>,
    /// A collection of references to taxes being returned for an order, including the total applied
    /// tax amount to be returned. The taxes must reference a top-level tax ID from the source
    /// order.
    pub return_taxes: Option<Vec<OrderReturnTax>>,
    /// A collection of references to discounts being returned for an order, including the total
    /// applied discount amount to be returned. The discounts must reference a top-level discount ID
    /// from the source order.
    pub return_discounts: Option<Vec<OrderReturnDiscount>>,
    /// A positive or negative rounding adjustment to the total value being returned. Adjustments
    /// are commonly used to apply cash rounding when the minimum unit of the account is smaller
    /// than the lowest physical denomination of the currency.
    pub rounding_adjustment: Option<OrderRoundingAdjustment>,
    /// An aggregate monetary value being returned by this return entry.
    pub return_amounts: Option<OrderMoneyAmounts>,
}
