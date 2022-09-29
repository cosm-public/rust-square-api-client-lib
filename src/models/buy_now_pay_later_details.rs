//! Model struct for BuyNowPayLaterDetails type.

use serde::{Deserialize, Serialize};

use super::AfterpayDetails;

/// Additional details about a Buy Now Pay Later payment type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BuyNowPayLaterDetails {
    /// The brand used for the Buy Now Pay Later payment. The brand can be `AFTERPAY` or `UNKNOWN`.
    pub brand: Option<String>,
    /// Details about an Afterpay payment. These details are only populated if the `brand` is
    /// `AFTERPAY`.
    pub afterpay_details: Option<AfterpayDetails>,
}
