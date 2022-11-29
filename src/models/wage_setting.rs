//! Model struct for WageSetting type

use serde::{Deserialize, Serialize};

use super::{DateTime, JobAssignment};

/// An object representing a team member's wage information.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct WageSetting {
    /// The unique ID of the `TeamMember` whom this wage setting describes.
    pub team_member_id: Option<String>,
    /// Required. The ordered list of jobs that the team member is assigned to. The first job
    /// assignment is considered the team member's primary job.
    ///
    /// The minimum length is 1 and the maximum length is 12.
    pub job_assignments: Vec<JobAssignment>,
    /// Whether the team member is exempt from the overtime rules of the seller's country.
    pub is_overtime_exempt: Option<bool>,
    /// Used for resolving concurrency issues. The request fails if the version provided does not
    /// match the server version at the time of the request. If not provided, Square executes a
    /// blind write, potentially overwriting data from another write. For more information, see
    /// [optimistic
    /// concurrency](https://developer.squareup.com/docs/working-with-apis/optimistic-concurrency).
    pub version: Option<i32>,
    /// **Read only** The timestamp, in RFC 3339 format, describing when the wage setting object was
    /// created.
    pub created_at: Option<DateTime>,
    /// **Read only** The timestamp, in RFC 3339 format, describing when the wage setting object was
    /// last updated.
    pub updated_at: Option<DateTime>,
}
