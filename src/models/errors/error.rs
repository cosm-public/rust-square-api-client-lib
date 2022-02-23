//! Model struct for Error type

use serde::Deserialize;

/// This is a model struct for Error type.
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq)]
pub struct Error {
    /// Indicates which high-level category of error has occurred during a request to the Connect
    /// API.
    pub category: String,
    /// Indicates the specific error that occurred during a request to a Square API.
    pub code: String,
    /// A human-readable description of the error for debugging purposes.
    pub detail: String,
    /// The name of the field provided in the original request (if any) that the error pertains to.
    pub field: String,
}

impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
