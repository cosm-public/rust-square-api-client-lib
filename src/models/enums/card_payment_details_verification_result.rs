//! Model for CardPaymentDetailsVerificationResult enum.

use serde::{Deserialize, Serialize};

/// Results of cardholder verification.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPaymentDetailsVerificationResult {
    Success,
    Failure,
    Unknown,
}
