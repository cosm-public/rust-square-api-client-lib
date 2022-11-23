//! Response body struct for the Update Subscription API

use serde::Deserialize;

use super::{errors::Error, Subscription};

/// This is a model struct for UpdateSubscriptionResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct UpdateSubscriptionResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The updated subscription.
    pub subscription: Option<Subscription>,
}
