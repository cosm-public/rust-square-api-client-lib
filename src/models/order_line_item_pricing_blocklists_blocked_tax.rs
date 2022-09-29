//! Model struct for OrderLineItemPricingBlocklistsBlockedTax type

use serde::{Deserialize, Serialize};

/// A tax to block from applying to a line item.
///
/// The tax must be identified by either `tax_uid` or `tax_catalog_object_id`, but not both.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderLineItemPricingBlocklistsBlockedTax {
    /// A unique ID of the `BlockedTax` within the order.
    pub uid: Option<String>,
    /// The `uid` of the tax that should be blocked. Use this field to block ad hoc taxes. For
    /// catalog, taxes use the `tax_catalog_object_id` field.
    pub tax_uid: Option<String>,
    /// The `catalog_object_id` of the tax that should be blocked. Use this field to block catalog
    /// taxes. For ad hoc taxes, use the `tax_uid` field.
    pub tax_catalog_object_id: Option<String>,
}
