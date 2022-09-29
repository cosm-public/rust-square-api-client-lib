//! Model struct for SearchOrdersStateFilter type

use serde::Serialize;

use super::enums::OrderState;

/// Filter by the current order `state`.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchOrdersStateFilter {
    /// States to filter for.
    pub states: Option<Vec<OrderState>>,
}
