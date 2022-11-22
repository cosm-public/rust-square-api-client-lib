//! Model struct for SubscriptionSource type

use serde::{Deserialize, Serialize};

/// The origination details of the subscription.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubscriptionSource {
    /// **Read only** The name used to identify the place (physical or digital) that a subscription
    /// originates. If unset, the name defaults to the name of the application that created the
    /// subscription.
    ///
    /// Max Length: 255
    pub name: Option<String>,
}
