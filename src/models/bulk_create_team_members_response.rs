//! Response body struct for the Bulk Create Team Members API

use std::collections::HashMap;

use serde::Deserialize;

use super::{errors::Error, CreateTeamMemberResponse};

/// This is a model struct for BulkCreateTeamMembersResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BulkCreateTeamMembersResponse {
    /// The successfully created `TeamMember` objects. Each key is the `idempotency_key` that maps
    /// to the `CreateTeamMemberRequest`.
    pub team_members: Option<HashMap<String, CreateTeamMemberResponse>>,
    /// The errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
