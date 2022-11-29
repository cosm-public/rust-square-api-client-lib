//! Model struct for SearchTeamMembersQuery type

use serde::Serialize;

use super::SearchTeamMembersFilter;

/// Represents the parameters in a search for `TeamMember` objects.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchTeamMembersQuery {
    /// The options to filter by.
    pub filter: Option<SearchTeamMembersFilter>,
}
