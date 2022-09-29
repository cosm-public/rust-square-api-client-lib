//! Model struct for OrderLineItemPricingBlocklists type

use serde::{Deserialize, Serialize};

use super::{
    OrderLineItemPricingBlocklistsBlockedDiscount, OrderLineItemPricingBlocklistsBlockedTax,
};

/// Describes pricing adjustments that are blocked from manual and automatic application to a line
/// item.
///
/// For more information, see [Apply Taxes and
/// Discounts](https://developer.squareup.com/docs/orders-api/apply-taxes-and-discounts).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderLineItemPricingBlocklists {
    /// A list of discounts blocked from applying to the line item. Discounts can be blocked by the
    /// `discount_uid` (for ad hoc discounts) or the `discount_catalog_object_id` (for catalog
    /// discounts).
    pub blocked_discounts: Option<Vec<OrderLineItemPricingBlocklistsBlockedDiscount>>,
    /// A list of taxes blocked from applying to the line item. Taxes can be blocked by the
    /// `tax_uid` (for ad hoc taxes) or the `tax_catalog_object_id` (for catalog taxes).
    pub blocked_taxes: Option<Vec<OrderLineItemPricingBlocklistsBlockedTax>>,
}
