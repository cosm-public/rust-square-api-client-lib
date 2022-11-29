//! Response body struct for the Retrieve Team Member API

use serde::Deserialize;

use super::{errors::Error, TeamMember};

/// This is a model struct for RetrieveTeamMemberResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RetrieveTeamMemberResponse {
    /// The successfully retrieved `TeamMember` object.
    pub team_member: Option<TeamMember>,
    /// The errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
