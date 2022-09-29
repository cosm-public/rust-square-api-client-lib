//! Model for SubscriptionCadence enum.

use serde::{Deserialize, Serialize};

/// Determines the billing cadence of a [Subscription]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionCadence {
    /// Once per day
    Daily,
    /// Once per week
    Weekly,
    /// Every two weeks
    EveryTwoWeeks,
    /// Once every 30 days
    ThirtyDays,
    /// Once every 60 days
    SixtyDays,
    /// Once every 90 days
    NinetyDays,
    /// Once per month
    Monthly,
    /// Once every two months
    EveryTwoMonths,
    /// Once every three months
    Quarterly,
    /// Once every four months
    EveryFourMonths,
    /// Once every six months
    EverySixMonths,
    /// Once per year
    Annual,
    /// Once every two years
    EveryTwoYears,
}

impl Default for SubscriptionCadence {
    fn default() -> Self {
        Self::Monthly
    }
}
