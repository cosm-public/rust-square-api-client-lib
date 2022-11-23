//! Response body struct for the Resume Subscription API

use serde::Deserialize;

use super::{errors::Error, Subscription, SubscriptionAction};

/// This is the model struct for the ResumeSubscriptionResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct ResumeSubscriptionResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The resumed subscription.
    pub subscription: Option<Subscription>,
    /// A list of `RESUME` actions created by the request and scheduled for the subscription.
    pub actions: Option<Vec<SubscriptionAction>>,
}
