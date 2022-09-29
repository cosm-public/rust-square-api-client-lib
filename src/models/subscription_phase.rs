//! Model struct for SubscriptionPhase type.

use serde::{Deserialize, Serialize};

use super::{enums::SubscriptionCadence, Money};

/// Describes a phase in a subscription plan.
///
/// For more information, see [Set Up and Manage a Subscription
/// Plan](https://developer.squareup.com/docs/subscriptions-api/setup-plan).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubscriptionPhase {
    /// The Square-assigned ID of the subscription phase. This field cannot be changed after a
    /// `SubscriptionPhase` is created.
    pub uid: Option<String>,
    /// The billing cadence of the phase. For example, weekly or monthly. This field cannot be
    /// changed after a `SubscriptionPhase` is created.
    pub cadence: SubscriptionCadence,
    /// The number of `cadence`s the phase lasts. If not set, the phase never ends. Only the last
    /// phase can be indefinite. This field cannot be changed after a `SubscriptionPhase` is
    /// created.
    pub periods: Option<i32>,
    /// The amount to bill for each `cadence`.
    pub recurring_price_money: Money,
    /// The position this phase appears in the sequence of phases defined for the plan, indexed from
    /// 0. This field cannot be changed after a `SubscriptionPhase` is created.
    pub ordinal: Option<i64>,
}
