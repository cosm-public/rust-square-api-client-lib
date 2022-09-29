//! Model for LocationStatus enum

use serde::{Deserialize, Serialize};

/// A location's status.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LocationStatus {
    /// A location that is active for business.
    Active,
    /// A location that is not active for business. Inactive locations provide historical
    /// information. Hide inactive locations unless the user has requested to see them.
    Inactive,
}
