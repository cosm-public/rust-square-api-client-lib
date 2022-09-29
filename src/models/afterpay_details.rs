//! Model struct for AfterpayDetails type.

use serde::{Deserialize, Serialize};

/// Additional details about Afterpay payments.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AfterpayDetails {
    /// Email address on the buyer's Afterpay account.
    pub email_address: Option<String>,
}
