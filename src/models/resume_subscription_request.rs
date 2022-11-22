//! Request body struct for the Resume Subscription API

use serde::Serialize;

use super::enums::ChangeTiming;

/// This is the model struct for the ResumeSubscriptionRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct ResumeSubscriptionRequest {
    /// The `YYYY-MM-DD`-formatted date when the subscription reactivated.
    pub resume_effective_date: Option<String>,
    /// The timing to resume a subscription, relative to the specified `resume_effective_date`
    /// attribute value.
    pub resume_change_timing: Option<ChangeTiming>,
}
