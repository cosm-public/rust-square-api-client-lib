//! Request body struct for the Bulk Update Team Members API

use std::collections::HashMap;

use serde::Serialize;

use super::UpdateTeamMemberRequest;

/// This is a model struct for BulkUpdateTeamMembersRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct BulkUpdateTeamMembersRequest {
    /// The data used to update the `TeamMember` objects. Each key is the `team_member_id` that
    /// maps to the `UpdateTeamMemberRequest`.
    pub team_members: HashMap<String, UpdateTeamMemberRequest>,
}
