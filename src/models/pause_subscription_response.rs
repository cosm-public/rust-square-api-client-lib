//! Response body struct for the Pause Subscription API

use serde::Deserialize;

use super::{errors::Error, Subscription, SubscriptionAction};

/// This is the model struct for the PauseSubscriptionResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct PauseSubscriptionResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The subscription to be paused by the scheduled `PAUSE` action.
    pub subscription: Option<Subscription>,
    /// The list of a `PAUSE` action and a possible `RESUME` action created by the request.
    pub actions: Option<Vec<SubscriptionAction>>,
}
