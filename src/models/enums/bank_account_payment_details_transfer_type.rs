//! Model for BankAccountPaymentDetailsTransferType type.

use serde::{Deserialize, Serialize};

/// A type of bank transfer
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankAccountPaymentDetailsTransferType {
    Ach,
    Unknown,
}
