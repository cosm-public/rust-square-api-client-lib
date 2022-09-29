//! Model struct for ExternalPaymentDetails type.

use serde::{Deserialize, Serialize};

use super::{enums::ExternalPaymentType, Money};

/// Stores details about an external payment.
///
/// Contains only non-confidential information. For more information, see [Take External
/// Payments](https://developer.squareup.com/docs/payments-api/take-payments/external-payments).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExternalPaymentDetails {
    /// The type of external payment the seller received.
    pub r#type: Option<ExternalPaymentType>,
    /// A description of the external payment source. For example, "Food Delivery Service".
    pub source: Option<String>,
    /// An ID to associate the payment to its originating source.
    pub source_id: Option<String>,
    /// The fees paid to the source. The `amount_money` minus this field is the net amount seller
    /// receives.
    pub source_fee_money: Option<Money>,
}
