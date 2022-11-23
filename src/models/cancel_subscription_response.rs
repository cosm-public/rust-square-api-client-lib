//! Response body struct for the Cancel Subscription API.

use serde::Deserialize;

use super::{errors::Error, Subscription, SubscriptionAction};

/// This is a model struct for CancelSubscriptionResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CancelSubscriptionResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The specified subscription scheduled for cancellation according to the action created by the
    /// request.
    pub subscription: Option<Subscription>,
    /// A list of a single `CANCEL` action scheduled for the subscription.
    pub actions: Option<Vec<SubscriptionAction>>,
}
