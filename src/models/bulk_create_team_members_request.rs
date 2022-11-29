//! Request body struct for the Bulk Create Team Members API

use std::collections::HashMap;

use serde::Serialize;

use super::CreateTeamMemberRequest;

/// This is a model struct for BulkCreateTeamMembersRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct BulkCreateTeamMembersRequest {
    /// The data used to create the `TeamMember` objects. Each key is the `idempotency_key` that
    /// maps to the `CreateTeamMemberRequest`.
    pub team_members: HashMap<String, CreateTeamMemberRequest>,
}
