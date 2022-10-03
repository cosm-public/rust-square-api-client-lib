//! Model struct for GiftCardActivityRedeem type

use serde::{Deserialize, Serialize};

use super::{enums::GiftCardActivityRedeemStatus, Money};

/// Represents details about a `REDEEM` [gift card activity type](GiftCardActivityType).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCardActivityRedeem {
    /// The amount deducted from the gift card for the redemption. This value is a positive integer.
    ///
    /// Applications that use a custom order processing system must specify this amount in the
    /// [CreateGiftCardActivity](https://developer.squareup.com/reference/square/giftcardactivities-api/create-gift-card-activity)
    /// request.
    pub amount_money: Money,
    /// **Read only** The ID of the payment that represents the gift card redemption. Square
    /// populates this field if the payment was processed by Square.
    pub payment_id: Option<String>,
    /// A client-specified ID that associates the gift card activity with an entity in another
    /// system.
    ///
    /// Applications that use a custom order processing system can use this field to track
    /// information related to an order or payment.
    pub reference_id: Option<String>,
    /// **Read only** The status of the gift card redemption. Gift cards redeemed from Square Point
    /// of Sale or the Square Seller Dashboard use a two-state process: `PENDING` to `COMPLETED` or
    /// `PENDING` to `CANCELED`. Gift cards redeemed using the Gift Card Activities API always have
    /// a `COMPLETED` status.
    pub status: Option<GiftCardActivityRedeemStatus>,
}
