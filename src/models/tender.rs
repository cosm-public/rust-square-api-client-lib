//! Model struct for Tender type

use serde::{Deserialize, Serialize};

use super::{
    enums::TenderType, AdditionalRecipient, DateTime, Money, TenderCardDetails, TenderCashDetails,
};

/// Represents a tender (i.e., a method of payment) used in a Square transaction.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Tender {
    /// The tender's unique ID.
    pub id: Option<String>,
    /// The ID of the transaction's associated location.
    pub location_id: Option<String>,
    /// The ID of the tender's associated transaction.
    pub transaction_id: Option<String>,
    /// **Read only** The timestamp for when the tender was created.
    pub created_at: Option<DateTime>,
    /// An optional note associated with the tender at the time of payment.
    pub note: Option<String>,
    /// The total amount of the tender, including `tip_money`. If the tender has a `payment_id`, the
    /// `total_money` of the corresponding [Payment] will be equal to the `amount_money` of the
    /// tender.
    pub amount_money: Option<Money>,
    /// The tip's amount of the tender.
    pub tip_money: Option<Money>,
    /// The amount of any Square processing fees applied to the tender.
    ///
    /// This field is not immediately populated when a new transaction is created. It is usually
    /// available after about ten seconds.
    pub processing_fee_money: Option<Money>,
    /// If the tender is associated with a customer or represents a customer's card on file, this is
    /// the ID of the associated customer.
    pub customer_id: Option<String>,
    /// **Required** The type of tender, such as `CARD` or `CASH`.
    pub r#type: TenderType,
    /// The details of the card tender.
    ///
    /// This value is present only if the value of `type` is `CARD`.
    pub card_details: Option<TenderCardDetails>,
    /// The details of the cash tender.
    ///
    /// This value is present only if the value of `type` is `CASH`.
    pub cash_details: Option<TenderCashDetails>,
    /// Additional recipients (other than the merchant) receiving a portion of this tender. For
    /// example, fees assessed on the purchase by a third party integration.
    #[deprecated]
    pub additional_recipients: Option<Vec<AdditionalRecipient>>,
    /// The ID of the [Payment] that corresponds to this tender. This value is only
    /// present for payments created with the v2 Payments API.
    pub payment_id: Option<String>,
}
