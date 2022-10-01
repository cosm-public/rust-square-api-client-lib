//! Model struct for GiftCardActivityImportReversal type

use serde::{Deserialize, Serialize};

use super::Money;

/// Represents details about an `IMPORT_REVERSAL` [gift card activity type](GiftCardActivityType).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCardActivityImportReversal {
    /// The amount of money cleared from the third-party gift card when the import was reversed.
    pub amount_money: Money,
}
