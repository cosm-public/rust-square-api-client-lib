//! Request struct for the Register Domain API

use serde::Serialize;

/// This is a model struct for RegisterDomainRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct RegisterDomainRequest {
    /// A domain name as described in RFC-1034 that will be registered with ApplePay.
    ///
    /// Min Length: 1, Max Length: 255
    pub domain_name: String,
}
