//! Model struct for AdditionalRecipient type

use serde::Deserialize;

use super::Money;

/// This is a model struct for AdditionalRecipient type.
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq)]
pub struct AdditionalRecipient {
    /// The location ID for a recipient (other than the merchant) receiving a portion of this
    /// tender.
    pub location_id: String,
    /// The description of the additional recipient.
    pub description: String,
    /// Represents an amount of money. `Money` fields can be signed or unsigned. Fields that do not
    /// explicitly define whether they are signed or unsigned are considered unsigned and can only
    /// hold positive amounts. For signed fields, the sign of the value indicates the purpose of the
    /// money transfer. See [Working with Monetary
    /// Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts) for
    /// more information.
    pub amount_money: Money,
    /// The unique ID for this [AdditionalRecipientReceivable]($m/AdditionalRecipientReceivable),
    /// assigned by the server.
    pub receivable_id: String,
}
