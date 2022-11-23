//! Model struct for SubscriptionEvent type

use serde::{Deserialize, Serialize};

use super::{enums::SubscriptionEventSubscriptionEventType, SubscriptionEventInfo};

/// Describes changes to a subscription and the subscription status.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubscriptionEvent {
    /// The ID of the subscription event.
    pub id: String,
    /// Type of the subscription event.
    pub subscription_event_type: SubscriptionEventSubscriptionEventType,
    /// The `YYYY-MM-DD`-formatted date (for example, 2013-01-15) when the subscription event
    /// occurred.
    pub effective_date: String,
    /// The ID of the subscription plan associated with the subscription.
    pub plan_id: String,
    /// Additional information about the subscription event.
    pub info: Option<SubscriptionEventInfo>,
}
