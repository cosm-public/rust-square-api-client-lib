//! Model struct for TenderCardDetails type

use serde::{Deserialize, Serialize};

use super::{
    enums::{TenderCardDetailsEntryMethod, TenderCardDetailsStatus},
    Card,
};

/// Represents additional details of a tender with type `CARD` or `SQUARE_GIFT_CARD`
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TenderCardDetails {
    /// The credit card payment's current state (such as `AUTHORIZED` or `CAPTURED`). See
    /// [TenderCardDetailsStatus] for possible values.
    pub status: Option<TenderCardDetailsStatus>,
    /// The credit card's non-confidential details.
    pub card: Option<Card>,
    /// The method used to enter the card's details for the transaction.
    pub entry_method: Option<TenderCardDetailsEntryMethod>,
}
