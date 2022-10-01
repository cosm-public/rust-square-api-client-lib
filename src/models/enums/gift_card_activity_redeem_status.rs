//! Model for GiftCardActivityRedeemStatus enum

use serde::{Deserialize, Serialize};

/// Indicates the status of a [gift card](GiftCard) redemption.
///
/// This status is relevant only for redemptions made from Square products (such as Square Point of
/// Sale) because Square products use a two-state process. Gift cards redeemed using the Gift Card
/// Activities API always have a `COMPLETED` status.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardActivityRedeemStatus {
    /// The gift card redemption is pending. `PENDING` is a temporary status that applies when a
    /// gift card is redeemed from Square Point of Sale or another Square product. A `PENDING`
    /// status is updated to `COMPLETED` if the payment is captured or `CANCELED` if the
    /// authorization is voided.
    Pending,
    /// The gift card redemption is completed.
    Completed,
    /// The gift card redemption is canceled. A redemption is canceled if the authorization on the
    /// gift card is voided.
    Canceled,
}
