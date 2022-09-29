//! Model struct for Error type

use serde::{Deserialize, Serialize};

use crate::models::enums::{ErrorCategory, ErrorCode};

/// Represents an error encountered during a request to the Connect API.
///
/// See [Handling errors](https://developer.squareup.com/docs/build-basics/handling-errors) for more
/// information.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Error {
    /// Indicates which high-level category of error has occurred during a request to the Connect
    /// API.
    pub category: ErrorCategory,
    /// Indicates the specific error that occurred during a request to a Square API.
    pub code: ErrorCode,
    /// A human-readable description of the error for debugging purposes.
    pub detail: String,
    /// The name of the field provided in the original request (if any) that the error pertains to.
    /// This is an optional field as it would be absent on errors like - Invalid Authentication token
    pub field: Option<String>,
}
