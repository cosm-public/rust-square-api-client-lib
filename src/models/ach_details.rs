//! Model struct for AchDetails type.

use serde::{Deserialize, Serialize};

use super::enums::AchDetailsAccountType;

/// ACH-specific details about `BANK_ACCOUNT` type payments with the `transfer_type` of `ACH`.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AchDetails {
    /// The routing number for the bank account.
    pub routing_number: Option<String>,
    /// The last few digits of the bank account number.
    pub account_number_suffix: Option<String>,
    /// The type of the bank account performing the transfer.
    pub account_type: Option<AchDetailsAccountType>,
}
