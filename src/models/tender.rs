//! Model struct for Tender type

use serde::Deserialize;

use super::{AdditionalRecipient, Money, TenderCardDetails, TenderCashDetails};

/// This is a model struct for Tender type.
#[derive(Debug, Default, Deserialize, Hash, PartialEq)]
pub struct Tender {
    /// The tender's unique ID.
    pub id: String,
    /// The ID of the transaction's associated location.
    pub location_id: String,
    /// The ID of the tender's associated transaction.
    pub transaction_id: String,
    /// The timestamp for when the tender was created, in RFC 3339 format.
    pub created_at: String,
    /// An optional note associated with the tender at the time of payment.
    pub note: String,
    /// Represents an amount of money. `Money` fields can be signed or unsigned. Fields that do not
    /// explicitly define whether they are signed or unsigned are considered unsigned and can only
    /// hold positive amounts. For signed fields, the sign of the value indicates the purpose of the
    /// money transfer. See [Working with Monetary
    /// Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts) for
    /// more information.
    pub amount_money: Money,
    /// Represents an amount of money. `Money` fields can be signed or unsigned. Fields that do not
    /// explicitly define whether they are signed or unsigned are considered unsigned and can only
    /// hold positive amounts. For signed fields, the sign of the value indicates the purpose of the
    /// money transfer. See [Working with Monetary
    /// Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts) for
    /// more information.
    pub tip_money: Money,
    /// Represents an amount of money. `Money` fields can be signed or unsigned. Fields that do not
    /// explicitly define whether they are signed or unsigned are considered unsigned and can only
    /// hold positive amounts. For signed fields, the sign of the value indicates the purpose of the
    /// money transfer. See [Working with Monetary
    /// Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts) for
    /// more information.
    pub processing_fee_money: Money,
    /// If the tender is associated with a customer or represents a customer's card on file, this is
    /// the ID of the associated customer.
    pub customer_id: String,
    /// Indicates a tender's type.
    pub tender_type: String,
    /// Represents additional details of a tender with `type` `CARD` or `SQUARE_GIFT_CARD`
    pub card_details: TenderCardDetails,
    /// Represents the details of a tender with `type` `CASH`.
    pub cash_details: TenderCashDetails,
    /// Additional recipients (other than the merchant) receiving a portion of this tender. For
    /// example, fees assessed on the purchase by a third party integration.
    pub additional_recipients: Vec<AdditionalRecipient>,
    /// The ID of the [Payment]($m/Payment) that corresponds to this tender. This value is only
    /// present for payments created with the v2 Payments API.
    pub payment_id: String,
}
