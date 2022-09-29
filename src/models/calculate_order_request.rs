//! Model struct for CalculateOrderRequest type

use serde::Serialize;

use super::{Order, OrderReward};

/// This is a model struct for CalculateOrderRequest type
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CalculateOrderRequest {
    /// The order to be calculated. Expects the entire order, not a sparse update.
    pub order: Order,
    /// Identifies one or more loyalty reward tiers to apply during the order calculation. The
    /// discounts defined by the reward tiers are added to the order only to preview the effect of
    /// applying the specified rewards. The rewards do not correspond to actual redemptions; that
    /// is, no `reward`s are created. Therefore, the reward `id`s are random strings used only to
    /// reference the reward tier.
    pub proposed_rewards: Option<Vec<OrderReward>>,
}
