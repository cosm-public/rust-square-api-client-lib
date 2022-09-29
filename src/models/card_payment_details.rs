//! Model struct for CardPaymentDetails type.

use serde::{Deserialize, Serialize};

use super::{
    enums::{
        CardPaymentDetailsAvsStatus, CardPaymentDetailsCvvStatus, CardPaymentDetailsEntryMethod,
        CardPaymentDetailsStatus, CardPaymentDetailsVerificationMethod,
        CardPaymentDetailsVerificationResult,
    },
    errors::Error,
    Card, CardPaymentTimeline, DeviceDetails,
};

/// Reflects the current status of a card payment.
///
/// Contains only non-confidential information.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CardPaymentDetails {
    /// The card payment's current state.
    pub status: Option<CardPaymentDetailsStatus>,
    /// The credit card's non-confidential details.
    pub card: Option<Card>,
    /// The method used to enter the card's details for the payment.
    pub entry_method: Option<CardPaymentDetailsEntryMethod>,
    /// The status code returned from the Card Verification Value (CVV) check.
    pub cvv_status: Option<CardPaymentDetailsCvvStatus>,
    /// The status code returned from the Address Verification System (AVS) check.
    pub avs_status: Option<CardPaymentDetailsAvsStatus>,
    /// The status code returned by the card issuer that describes the payment's authorization
    /// status.
    pub auth_result_code: Option<String>,
    /// For EMV payments, the application ID identifies the EMV application used for the payment.
    pub application_identifier: Option<String>,
    /// For EMV payments, the human-readable name of the EMV application used for the payment.
    pub application_name: Option<String>,
    /// For EMV payments, the cryptogram generated for the payment.
    pub application_cryptogram: Option<String>,
    /// For EMV payments, the method used to verify the cardholder's identity.
    pub verification_method: Option<CardPaymentDetailsVerificationMethod>,
    /// For EMV payments, the results of the cardholder verification.
    pub verfication_results: Option<CardPaymentDetailsVerificationResult>,
    /// The statement description sent to the card networks.
    ///
    /// Note: The actual statement description varies and is likely to be truncated and appended
    /// with additional information on a per issuer basis.
    pub statement_description: Option<String>,
    /// **Deprecated:** Use `Payment.device_details` instead.
    ///
    /// Details about the device that took the payment.
    #[deprecated]
    pub device_details: Option<DeviceDetails>,
    /// The timeline for card payments.
    pub card_payment_timeline: Option<CardPaymentTimeline>,
    /// Whether the card must be physically present for the payment to be refunded. If set to
    /// `true`, the card must be present.
    pub refund_requires_card_presence: Option<bool>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
