//! Model struct for OrderLineItemDiscount type

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    enums::{OrderLineItemDiscountScope, OrderLineItemDiscountType},
    Money,
};

/// Represents a discount that applies to one or more line items in an order.
///
/// Fixed-amount, order-scoped discounts are distributed across all non-zero line item totals. The
/// amount distributed to each line item is relative to the amount contributed by the item to the
/// order subtotal.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OrderLineItemDiscount {
    /// A unique ID that identifies the discount only within this order.
    pub uid: Option<String>,
    /// The catalog object ID referencing [CatalogDiscount].
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this discount references.
    pub catalog_version: Option<i64>,
    /// The discount's name.
    pub name: Option<String>,
    /// The type of the discount.
    ///
    /// Discounts that do not reference a catalog object ID must have a type of `FIXED_PERCENTAGE`
    /// or `FIXED_AMOUNT`.
    #[serde(rename = "type")]
    pub order_line_item_discount_type: Option<OrderLineItemDiscountType>,
    /// The percentage of the discount, as a string representation of a decimal number. A value of
    /// `7.25` corresponds to a percentage of 7.25%.
    ///
    /// `percentage` is not set for amount-based discounts.
    pub percentage: Option<String>,
    /// The total declared monetary amount of the discount.
    ///
    /// `amount_money` is not set for percentage-based discounts.
    pub amount_money: Option<Money>,
    /// The amount of discount actually applied to the line item.
    ///
    /// The amount represents the amount of money applied as a line-item scoped discount. When an
    /// amount-based discount is scoped to the entire order, the value of `applied_money` is
    /// different than `amount_money` because the total amount of the discount is distributed across
    /// all line items.
    pub applied_money: Option<Money>,
    /// Application-defined data attached to this discount. Metadata fields are intended to store
    /// descriptive references or associations with an entity in another system or store brief
    /// information about the object. Square does not process this field; it only stores and returns
    /// it in relevant API calls. Do not use metadata to store any sensitive information (such as
    /// personally identifiable information or card details).
    ///
    /// Keys written by applications must be 60 characters or less and must be in the character set
    /// `[a-zA-Z0-9_-]`. Entries can also include metadata generated by Square. These keys are
    /// prefixed with a namespace, separated from the key with a ':' character.
    ///
    /// Values have a maximum length of 255 characters.
    ///
    /// An application can have up to 10 entries per metadata field.
    ///
    /// Entries written by applications are private and can only be read or modified by the same
    /// application.
    ///
    /// For more information,
    /// see [Metadata](https://developer.squareup.com/docs/build-basics/metadata).
    pub metadata: Option<HashMap<String, String>>,
    /// Indicates the level at which the discount applies. For `ORDER` scoped discounts, Square
    /// generates references in `applied_discounts` on all order line items that do not have them.
    /// For `LINE_ITEM` scoped discounts, the discount only applies to line items with a discount
    /// reference in their `applied_discounts` field.
    ///
    /// This field is immutable. To change the scope of a discount, you must delete the discount and
    /// re-add it as a new discount.
    pub scope: Option<OrderLineItemDiscountScope>,
    /// **Read only** The reward IDs corresponding to this discount. The application and
    /// specification of discounts that have `reward_ids` are completely controlled by the backing
    /// criteria corresponding to the reward tiers of the rewards that are added to the order
    /// through the Loyalty API. To manually unapply discounts that are the result of added rewards,
    /// the rewards must be removed from the order through the Loyalty API.
    pub reward_ids: Option<Vec<String>>,
    /// **Read only** The object ID of a [pricing rule](CatalogPricingRule) to be applied
    /// automatically to this discount. The specification and application of the discounts, to which
    /// a `pricing_rule_id` is assigned, are completely controlled by the corresponding pricing
    /// rule.
    pub pricing_rule_id: Option<String>,
}
