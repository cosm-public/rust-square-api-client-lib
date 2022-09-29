//! Model for TenderCardDetailsEntryMethod enum

use serde::{Deserialize, Serialize};

/// Indicates the method used to enter the card's details.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderCardDetailsEntryMethod {
    /// The card was swiped through a Square reader or Square stand.
    Swiped,
    /// The card information was keyed manually into Square Point of Sale or a Square-hosted web
    /// form.
    Keyed,
    /// The card was processed via EMV with a Square reader.
    Emv,
    /// The buyer's card details were already on file with Square.
    OnFile,
    /// The card was processed via a contactless (i.e., NFC) transaction with a Square reader.
    Contactless,
}
