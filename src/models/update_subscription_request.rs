//! Request body struct for the Update Subscription API

use serde::Serialize;

use super::Subscription;

/// This is a model struct for UpdateSubscriptionRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateSubscriptionRequest {
    /// The subscription object containing the current version, and fields to update. Unset fields
    /// will be left at their current server values, and JSON `null` values will be treated as a
    /// request to clear the relevant data.
    pub subscription: Option<Subscription>,
}
