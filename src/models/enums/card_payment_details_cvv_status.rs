//! Model for CardPaymentDetailsCvvStatus enum.

use serde::{Deserialize, Serialize};

/// Status code returned from the Card Verification Value (CVV) check.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPaymentDetailsCvvStatus {
    CvvAccepted,
    CvvRejected,
    CvvNotChecked,
}
