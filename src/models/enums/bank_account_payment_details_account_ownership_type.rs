//! Model for BankAccountPaymentDetailsAccountOwnershipType enum.

use serde::{Deserialize, Serialize};

/// An ownership type of a bank account performing a transfer for payment.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankAccountPaymentDetailsAccountOwnershipType {
    Individual,
    Company,
    Unknown,
}
