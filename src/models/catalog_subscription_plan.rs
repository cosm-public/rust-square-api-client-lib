//! Model struct for CatalogSubscriptionPlan type.

use serde::{Deserialize, Serialize};

use super::SubscriptionPhase;

/// Describes a subscription plan.
///
/// For more information, see [Set Up and Manage a Subscription
/// Plan](https://developer.squareup.com/docs/subscriptions-api/setup-plan).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogSubscriptionPlan {
    /// The name of the plan.
    pub name: String,
    /// A list of SubscriptionPhase containing the [SubscriptionPhase] for this plan.
    pub phases: Vec<SubscriptionPhase>,
}
