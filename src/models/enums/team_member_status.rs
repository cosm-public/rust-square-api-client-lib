//! Model for TeamMemberStatus enum.

use serde::{Deserialize, Serialize};

/// Enumerates the possible statuses the team member can have within a business.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TeamMemberStatus {
    /// The team member can sign in to Point of Sale and the Seller Dashboard.
    Active,
    /// The team member can no longer sign in to Point of Sale or the Seller Dashboard, but the team
    /// member's sales reports remain available.
    Inactive,
}
