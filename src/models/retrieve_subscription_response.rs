//! Response body struct for the Retrieve Subscription API

use serde::Deserialize;

use super::{errors::Error, Subscription};

/// This is a model struct for the RetrieveSubscriptionResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RetrieveSubscriptionResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The subscription retrieved.
    pub subscription: Option<Subscription>,
}
