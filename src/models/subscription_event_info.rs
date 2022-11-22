//! Model struct for SubscriptionEventInfo type

use serde::{Deserialize, Serialize};

use super::enums::SubscriptionEventInfoCode;

/// Provides information about the subscription event.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubscriptionEventInfo {
    /// A human-readable explanation for the event.
    pub detail: Option<String>,
    /// An info code indicating the subscription event that occurred.
    pub code: Option<SubscriptionEventInfoCode>,
}
