//! Model for CardPaymentDetailsAvsStatus enum.

use serde::{Deserialize, Serialize};

/// Status code returned from the Address Verification System (AVS) check.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPaymentDetailsAvsStatus {
    AvsAccepted,
    AvsRejected,
    AvsNotChecked,
}
