//! Response struct for the Create Subscription API

use serde::Deserialize;

use super::{errors::Error, Subscription};

/// This is a model struct for CreateSubscriptionResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CreateSubscriptionResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The newly created subscription.
    ///
    /// For more information, see [Subscription
    /// object](https://developer.squareup.com/docs/subscriptions-api/overview#subscription-object).
    pub subscription: Subscription,
}
