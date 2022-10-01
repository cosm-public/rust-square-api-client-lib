//! Model struct for GiftCardActivity type

use serde::{Deserialize, Serialize};

use super::{
    enums::GiftCardActivityType, DateTime, GiftCardActivityActivate,
    GiftCardActivityAdjustDecrement, GiftCardActivityAdjustIncrement, GiftCardActivityBlock,
    GiftCardActivityClearBalance, GiftCardActivityDeactivate, GiftCardActivityImport,
    GiftCardActivityImportReversal, GiftCardActivityLoad, GiftCardActivityRedeem,
    GiftCardActivityRefund, GiftCardActivityUnblock, GiftCardActivityUnlinkedActivityRefund, Money,
};

/// Represents an action performed on a gift card that affects its state or balance.
///
/// A gift card activity contains information about a specific activity type. For example, a
/// `REDEEM` activity includes a `redeem_activity_details` field that contains information about the
/// redemption.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCardActivity {
    /// **Read only** The Square-assigned ID of the gift card activity.
    pub id: Option<String>,
    /// The ID of the [business location](Location) where the activity occurred.
    pub location_id: String,
    /// The type of gift card activity.
    pub r#type: GiftCardActivityType,
    /// Additional details about an `ACTIVATE` activity, which is used to activate a gift card with
    /// an initial balance.
    pub activate_activity_details: Option<GiftCardActivityActivate>,
    /// Additional details about an `ADJUST_DECREMENT` activity, which is used to deduct money from
    /// a gift card outside of a typical `REDEEM` activity flow.
    pub adjust_decrement_activity_details: Option<GiftCardActivityAdjustDecrement>,
    /// Additional details about an `ADJUST_INCREMENT` activity, which is used to add money to a
    /// gift card outside of a typical `ACTIVATE`, `LOAD`, or `REFUND` activity flow.
    pub adjust_increment_activity_details: Option<GiftCardActivityAdjustIncrement>,
    /// **Read only** Additional details about a `BLOCK` activity, which Square uses to temporarily
    /// block a gift card.
    pub block_activity_details: Option<GiftCardActivityBlock>,
    /// Additional details about a `CLEAR_BALANCE` activity, which is used to set the balance of a
    /// gift card to zero.
    pub clear_balance_activity_details: Option<GiftCardActivityClearBalance>,
    /// **Read only** The timestamp when the gift card activity was created, in RFC 3339 format.
    pub created_at: Option<DateTime>,
    /// Additional details about a `DEACTIVATE` activity, which is used to deactivate a gift card.
    pub deactivate_activity_details: Option<GiftCardActivityDeactivate>,
    /// **Read only** The final balance on the gift card after the action is completed.
    pub gift_card_balance_money: Option<Money>,
    /// The gift card account number (GAN). When creating a gift card activity, `gift_card_gan` is
    /// not required if `gift_card_id` is specified.
    pub gift_card_gan: Option<String>,
    /// The gift card ID. When creating a gift card activity, `gift_card_id` is not required if
    /// `gift_card_gan` is specified.
    pub gift_card_id: Option<String>,
    /// **Read only** Additional details about an `IMPORT` activity, which Square uses to import a
    /// third-party gift card with a balance.
    pub import_activity_details: Option<GiftCardActivityImport>,
    /// **Read only** Additional details about an `IMPORT_REVERSAL` activity, which Square uses to
    /// reverse the import of a third-party gift card.
    pub import_reversal_activity_details: Option<GiftCardActivityImportReversal>,
    /// Additional details about a `LOAD` activity, which is used to reload money onto a gift card.
    pub load_activity_details: Option<GiftCardActivityLoad>,
    /// Additional details about a `REDEEM` activity, which is used to redeem a gift card for a
    /// purchase.
    ///
    /// For applications that process payments using the Square Payments API, Square creates a
    /// `REDEEM` activity that updates the gift card balance after the corresponding
    /// [CreatePayment](https://developer.squareup.com/reference/square/payments-api/create-payment)
    /// request is completed. Applications that use a custom payment processing system must call
    /// [CreateGiftCardActivity](https://developer.squareup.com/reference/square/giftcardactivities-api/create-gift-card-activity)
    /// to create the `REDEEM` activity.
    pub redeem_activity_details: Option<GiftCardActivityRedeem>,
    /// Additional details about a `REFUND` activity, which is used to add money to a gift card when
    /// refunding a payment.
    ///
    ///For applications that process payments using the Square Payments API, Square creates a
    /// `REFUND` activity that updates the gift card balance after the corresponding
    /// [RefundPayment](https://developer.squareup.com/reference/square/refunds-api/refund-payment)
    /// request is completed. Applications that use a custom payment processing system must call
    /// [CreateGiftCardActivity](https://developer.squareup.com/reference/square/giftcardactivities-api/create-gift-card-activity)
    /// to create the `REFUND` activity.
    pub refund_activity_details: Option<GiftCardActivityRefund>,
    /// **Read only** Additional details about an `UNBLOCK` activity, which Square uses to unblock a
    /// gift card.
    pub unblock_activity_details: Option<GiftCardActivityUnblock>,
    /// Additional details about an `UNLINKED_ACTIVITY_REFUND` activity. This activity is used to
    /// add money to a gift card when refunding a payment that was processed using a custom payment
    /// processing system and not linked to the gift card.
    pub unlinked_activity_refund_activity_details: Option<GiftCardActivityUnlinkedActivityRefund>,
}
