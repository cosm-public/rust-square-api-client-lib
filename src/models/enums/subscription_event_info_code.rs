//! Model for SubscriptionEventInfoCode enum.

use serde::{Deserialize, Serialize};

/// Supported info codes of a subscription event.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionEventInfoCode {
    /// The location is not active.
    LocationNotActive,
    /// The location cannot accept payments.
    LocationCannotAcceptPayment,
    /// The subscribing customer profile has been deleted.
    CustomerDeleted,
    /// The subscribing customer does not have an email.
    CustomerNoEmail,
    /// The subscribing customer does not have a name.
    CustomerNoName,
    /// User-provided detail.
    UserProvided,
}
