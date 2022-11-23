//! Request body struct for the Pause Subscription API

use serde::Serialize;

use super::enums::ChangeTiming;

/// This is the model struct for the PauseSubscriptionRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct PauseSubscriptionRequest {
    /// The `YYYY-MM-DD`-formatted date when the scheduled `PAUSE` action takes place on the
    /// subscription.
    ///
    /// When this date is unspecified or falls within the current billing cycle, the subscription is
    /// paused on the starting date of the next billing cycle.
    pub pause_effective_date: Option<String>,
    /// The number of billing cycles the subscription will be paused before it is reactivated.
    ///
    /// When this is set, a `RESUME` action is also scheduled to take place on the subscription at
    /// the end of the specified pause cycle duration. In this case, neither `resume_effective_date`
    /// nor `resume_change_timing` may be specified.
    pub pause_cycle_duration: Option<i64>,
    /// The date when the subscription is reactivated by a scheduled `RESUME` action. This date must
    /// be at least one billing cycle ahead of `pause_effective_date`.
    pub resume_effective_date: Option<String>,
    /// The timing whether the subscription is reactivated immediately or at the end of the billing
    /// cycle, relative to `resume_effective_date`.
    pub resume_change_timing: Option<ChangeTiming>,
    /// The user-provided reason to pause the subscription.
    pub pause_reason: Option<String>,
}
