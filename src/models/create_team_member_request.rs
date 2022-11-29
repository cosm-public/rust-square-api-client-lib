//! Request body struct for the Create Team Member API

use serde::Serialize;

use super::TeamMember;

/// This is a model struct for CreateTeamMemberRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CreateTeamMemberRequest {
    /// A unique string that identifies this `CreateTeamMember` request. Keys can be any valid
    /// string, but must be unique for every request. For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/basics/api101/idempotency).
    ///
    /// The minimum length is 1 and the maximum length is 45.
    pub idempotency_key: Option<String>,
    /// **Required** The data which will be used to create the `TeamMember` object.
    pub team_member: TeamMember,
}
