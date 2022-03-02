//! Response struct for the Create Card API

use serde::Deserialize;

use super::errors::Error;
use super::Transaction;

/// This is a model class for ChargeResponse type.
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq)]
pub struct CreateCardResponse {
    /// Any errors that occurred during the request.
    pub errors: Vec<Error>,
    /// Represents a transaction processed with Square, either with the Connect API or with Square
    /// Point of Sale. The `tenders` field of this object lists all methods of payment used to pay
    /// in the transaction.
    pub transaction: Transaction,
}
