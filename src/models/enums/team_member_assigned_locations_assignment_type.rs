//! Model for TeamMemberAssignedLocationsAssignmentType enum.

use serde::{Deserialize, Serialize};

/// Enumerates the possible assignment types that the team member can have.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TeamMemberAssignedLocationsAssignmentType {
    /// The team member is assigned to all current and future locations. The `location_ids` field is
    /// empty if the team member has this assignment type.
    AllCurrentAndFutureLocations,
    /// The team member is assigned to an explicit subset of locations. The `location_ids` field is
    /// the list of locations that the team member is assigned to.
    ExplicitLocations,
}
