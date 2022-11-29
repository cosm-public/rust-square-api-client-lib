//! Response body struct for the Bulk Update Team Members API

use std::collections::HashMap;

use serde::Deserialize;

use super::{errors::Error, UpdateTeamMemberResponse};

/// This is a model struct for BulkUpdateTeamMembersResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BulkUpdateTeamMembersResponse {
    /// The successfully updated `TeamMember` objects. Each key is the `team_member_id` that maps
    /// to the `UpdateTeamMemberRequest`.
    pub team_members: Option<HashMap<String, UpdateTeamMemberResponse>>,
    /// The errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
