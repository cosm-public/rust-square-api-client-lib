//! Response struct for the Register Domain API

use serde::Deserialize;

use super::{enums::RegisterDomainResponseStatus, errors::Error};

/// This is a model struct for RegisterDomainResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RegisterDomainResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The status of the domain registration.
    pub status: Option<RegisterDomainResponseStatus>,
}
