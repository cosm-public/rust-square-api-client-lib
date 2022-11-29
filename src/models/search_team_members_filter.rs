//! Model struct for SearchTeamMembersFilter type

use serde::Serialize;

use super::enums::TeamMemberStatus;

/// Represents a filter used in a search for TeamMember objects.
///
/// `AND` logic is applied between the individual fields, and `OR` logic is applied within
/// list-based fields. For example, setting this filter value:
///
/// ```
/// use square_api_client::models::{enums::TeamMemberStatus, SearchTeamMembersFilter};
///
/// let filter = SearchTeamMembersFilter {
///     location_ids: Some(vec![String::from("A"), String::from("B")]),
///     status: Some(TeamMemberStatus::Active),
///     ..Default::default()
/// };
/// ```
/// returns only active team members assigned to either location "A" or "B".
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchTeamMembersFilter {
    /// When present, filters by team members assigned to the specified locations. When empty,
    /// includes team members assigned to any location.
    pub location_ids: Option<Vec<String>>,
    /// When present, filters by team members who match the given status. When empty, includes team
    /// members of all statuses.
    pub status: Option<TeamMemberStatus>,
    /// When present and set to true, returns the team member who is the owner of the Square
    /// account.
    pub is_owner: Option<bool>,
}
