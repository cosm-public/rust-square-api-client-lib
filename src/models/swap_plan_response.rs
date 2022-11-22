//! Response body struct for the Swap Plan API

use serde::Deserialize;

use super::{errors::Error, Subscription, SubscriptionAction};

/// This is a model struct for the SwapPlanResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct SwapPlanResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The subscription with the updated subscription plan.
    pub subscription: Option<Subscription>,
    /// A list of a `SWAP_PLAN` action created by the request.
    pub actions: Option<Vec<SubscriptionAction>>,
}
