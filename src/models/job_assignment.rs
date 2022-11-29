//! Model struct for JobAssignment type

use serde::{Deserialize, Serialize};

use super::{enums::JobAssignmentPayType, Money};

/// An object describing a job that a team member is assigned to.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct JobAssignment {
    /// The title of the job.
    ///
    /// Min Length: 1
    pub job_title: String,
    /// The current pay type for the job assignment used to calculate the pay amount in a pay
    /// period.
    pub pay_type: JobAssignmentPayType,
    /// The hourly pay rate of the job.
    pub hourly_rate: Option<Money>,
    /// The total pay amount for a 12-month period on the job. Set if the job `PayType` is `SALARY`.
    pub annual_rate: Option<Money>,
    /// The planned hours per week for the job. Set if the job `PayType` is `SALARY`.
    pub weekly_hours: Option<i32>,
}
