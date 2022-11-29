//! Model for JobAssignmentPayType enum.

use serde::{Deserialize, Serialize};

/// Enumerates the possible pay types that a job can be assigned.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobAssignmentPayType {
    /// The job does not have a defined pay type.
    None,
    /// The job pays an hourly rate.
    Hourly,
    /// The job pays an annual salary.
    Salary,
}

impl Default for JobAssignmentPayType {
    fn default() -> Self {
        Self::None
    }
}
