//! Model struct for GiftCardActivityImport type

use serde::{Deserialize, Serialize};

use super::Money;

/// Represents details about an `IMPORT` [gift card activity type](GiftCardActivityType).
///
/// This activity type is used when Square imports a third-party gift card, in which case the
/// `gan_source` of the gift card is set to `OTHER`.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCardActivityImport {
    /// The balance amount on the imported gift card.
    pub amount_money: Money,
}
