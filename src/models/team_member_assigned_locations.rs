//! Model struct for TeamMemberAssignedLocations type

use serde::{Deserialize, Serialize};

use super::enums::TeamMemberAssignedLocationsAssignmentType;

/// A record representing an individual team member for a business.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TeamMemberAssignedLocations {
    /// The current assignment type of the team member.
    pub assignment_type: Option<TeamMemberAssignedLocationsAssignmentType>,
    /// The locations that the team member is assigned to.
    pub location_ids: Option<Vec<String>>,
}
