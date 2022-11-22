//! Model for SubscriptionActionType enum.

use serde::{Deserialize, Serialize};

/// Supported types of an action as a pending change to a subscription.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionActionType {
    /// The action to execute a scheduled cancellation of a subscription.
    Cancel,
    /// The action to execute a scheduled pause of a subscription.
    Pause,
    /// The action to execute a scheduled resumption of a subscription.
    Resume,
    /// The action to execute a scheduled swap of a subscription plan in a subscription.
    SwapPlan,
}
