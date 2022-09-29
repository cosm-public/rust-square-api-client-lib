//! Model struct for AdditionalRecipient type

use serde::{Deserialize, Serialize};

use super::Money;

/// Represents an additional recipient (other than the merchant) receiving a portion of this tender.
#[deprecated]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdditionalRecipient {
    /// **Required** The location ID for a recipient (other than the merchant) receiving a portion
    /// of this tender.
    pub location_id: String,
    /// The description of the additional recipient.
    pub description: Option<String>,
    /// **Required** The amount of money distributed to the recipient.
    pub amount_money: Money,
    /// The unique ID for this [AdditionalRecipientReceivable], assigned by the server.
    pub receivable_id: Option<String>,
}
