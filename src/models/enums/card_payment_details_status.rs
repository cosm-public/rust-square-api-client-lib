//! Model for CardPaymentDetailsStatus enum.

use serde::{Deserialize, Serialize};

/// Card payment state.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPaymentDetailsStatus {
    Authorized,
    Captured,
    Voided,
    Failed,
}
