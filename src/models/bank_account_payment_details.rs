//! Model struct for BankAccountPaymentDetails type.

use serde::{Deserialize, Serialize};

use super::{
    enums::{
        BankAccountPaymentDetailsAccountOwnershipType, BankAccountPaymentDetailsTransferType,
        Country,
    },
    errors::Error,
    AchDetails,
};

/// Additional details about BANK_ACCOUNT type payments.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BankAccountPaymentDetails {
    /// The name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// The type of the bank transfer.
    pub transfer_type: Option<BankAccountPaymentDetailsTransferType>,
    /// The ownership type of the bank account performing the transfer.
    pub account_ownership_type: Option<BankAccountPaymentDetailsAccountOwnershipType>,
    /// Uniquely identifies the bank account for this seller and can be used to determine if
    /// payments are from the same bank account.
    pub fingerprint: Option<String>,
    /// The two-letter ISO code representing the country the bank account is located in.
    pub country: Option<Country>,
    /// The statement description as sent to the bank.
    pub statement_description: Option<String>,
    /// ACH-specific information about the transfer. The information is only populated if the
    /// `transfer_type` is ACH.
    pub ach_details: Option<AchDetails>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
