//! Model struct for OrderLineItemPricingBlocklistsBlockedDiscount type

use serde::{Deserialize, Serialize};

/// A discount to block from applying to a line item.
///
/// The discount must be identified by either `discount_uid` or `discount_catalog_object_id`, but
/// not both.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderLineItemPricingBlocklistsBlockedDiscount {
    /// A unique ID of the `BlockedDiscount` within the order.
    pub uid: Option<String>,
    /// The `uid` of the discount that should be blocked. Use this field to block ad hoc discounts.
    /// For catalog discounts, use the `discount_catalog_object_id` field.
    pub discount_uid: Option<String>,
    /// The `catalog_object_id` of the discount that should be blocked. Use this field to block
    /// catalog discounts. For ad hoc discounts, use the `discount_uid` field.
    pub discount_catalog_object_id: Option<String>,
}
