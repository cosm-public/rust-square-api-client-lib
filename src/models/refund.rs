//! Model struct for Refund type

use serde::Deserialize;

use super::{AdditionalRecipient, Money};

/// This is a model struct for Refund type.
#[derive(Debug, Default, Deserialize, Hash, PartialEq)]
pub struct Refund {
    /// The refund's unique ID.
    pub id: String,
    /// The ID of the refund's associated location.
    pub location_id: String,
    /// The ID of the transaction that the refunded tender is part of.
    pub transaction_id: String,
    /// The ID of the refunded tender.
    pub tender_id: String,
    /// The timestamp for when the refund was created, in RFC 3339 format.
    pub created_at: String,
    /// The reason for the refund being issued.
    pub reason: String,
    /// Represents an amount of money. `Money` fields can be signed or unsigned. Fields that do not
    /// explicitly define whether they are signed or unsigned are considered unsigned and can only
    /// hold positive amounts. For signed fields, the sign of the value indicates the purpose of the
    /// money transfer. See [Working with Monetary
    /// Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts) for
    /// more information.
    pub amount_money: Money,
    /// Indicates a refund's current status.
    pub status: String,
    /// Represents an amount of money. `Money` fields can be signed or unsigned. Fields that do not
    /// explicitly define whether they are signed or unsigned are considered unsigned and can only
    /// hold positive amounts. For signed fields, the sign of the value indicates the purpose of the
    /// money transfer. See [Working with Monetary
    /// Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts) for
    /// more information.
    pub processing_fee_money: Money,
    /// Additional recipients (other than the merchant) receiving a portion of this refund. For
    /// example, fees assessed on a refund of a purchase by a third party integration.
    pub additional_recipients: Vec<AdditionalRecipient>,
}
