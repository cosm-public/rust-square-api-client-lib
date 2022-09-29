//! Model for CardPaymentDetailsVerificationMethod enum.

use serde::{Deserialize, Serialize};

/// Method to verify cardholder's identity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPaymentDetailsVerificationMethod {
    Pin,
    Signature,
    PinAndSignature,
    OnDevice,
    None,
}
