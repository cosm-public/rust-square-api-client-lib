//! Model for GiftCardActivityType enum

use serde::{Deserialize, Serialize};

/// Indicates the type of the [gift card activity](GiftCardActivity).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardActivityType {
    /// Activated a gift card with a balance. When a gift card is activated, Square changes the gift
    /// card state from `PENDING` to `ACTIVE`. A gift card must be in the `ACTIVE` state to be used
    /// for other balance-changing activities.
    Activate,
    /// Loaded a gift card with additional funds.
    Load,
    /// Redeemed a gift card for a purchase.
    Redeem,
    /// Set the balance of a gift card to zero.
    ClearBalance,
    /// Permanently blocked a gift card from balance-changing activities.
    Deactivate,
    /// Added money to a gift card outside of a typical `ACTIVATE`, `LOAD`, or `REFUND` activity
    /// flow.
    AdjustIncrement,
    /// Deducted money from a gift card outside of a typical `REDEEM` activity flow.
    AdjustDecrement,
    /// Added money to a gift card from a refunded transaction. A `REFUND` activity might be linked
    /// to a Square payment, depending on how the payment and refund are processed. For example:
    ///
    /// - A gift card payment processed by Square can be refunded to the same gift card using
    /// Square Point of Sale, the Square Seller Dashboard, or the Refunds API.
    /// - A cross-tender payment processed by Square can be refunded to a gift card using Square
    /// Point of Sale or the Square Seller Dashboard. The payment source might be a credit card or
    /// different gift card.
    /// - A payment processed using a custom payment processing system can be refunded to the same
    /// gift card.
    Refund,
    /// Added money to a gift card from a refunded transaction that was processed using a custom
    /// payment processing system and not linked to the gift card.
    UnlinkedActivityRefund,
    /// Imported a third-party gift card with a balance. `IMPORT` activities are managed by Square
    /// and cannot be created using the Gift Card Activities API.
    Import,
    /// Temporarily blocked a gift card from balance-changing activities. `BLOCK` activities are
    /// managed by Square and cannot be created using the Gift Card Activities API.
    Block,
    /// Unblocked a gift card, which enables it to resume balance-changing activities. `UNBLOCK`
    /// activities are managed by Square and cannot be created using the Gift Card Activities API.
    Unblock,
    /// Reversed the import of a third-party gift card, which sets the gift card state to `PENDING`
    /// and clears the balance. `IMPORT_REVERSAL` activities are managed by Square and cannot be
    /// created using the Gift Card Activities API.
    ImportReversal,
}

impl Default for GiftCardActivityType {
    fn default() -> Self {
        Self::Activate
    }
}
