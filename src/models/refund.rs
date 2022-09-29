//! Model struct for Refund type

use serde::{Deserialize, Serialize};

use super::{enums::RefundStatus, AdditionalRecipient, DateTime, Money};

/// Represents a refund processed for a Square transaction.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Refund {
    /// **Required** The refund's unique ID.
    pub id: String,
    /// **Required** The ID of the refund's associated location.
    pub location_id: String,
    /// **Required** The ID of the transaction that the refunded tender is part of.
    pub transaction_id: String,
    /// **Required** The ID of the refunded tender.
    pub tender_id: String,
    /// **Read only** The timestamp for when the refund was created.
    pub created_at: Option<DateTime>,
    /// **Required** The reason for the refund being issued.
    pub reason: String,
    /// **Required** The amount of money refunded to the buyer.
    pub amount_money: Money,
    /// **Required** The current status of the refund (`PENDING`, `APPROVED`, `REJECTED`, or
    /// `FAILED`).
    pub status: RefundStatus,
    /// The amount of Square processing fee money refunded to the merchant.
    pub processing_fee_money: Option<Money>,
    /// Additional recipients (other than the merchant) receiving a portion of this refund. For
    /// example, fees assessed on a refund of a purchase by a third party integration.
    #[deprecated]
    pub additional_recipients: Option<Vec<AdditionalRecipient>>,
}
