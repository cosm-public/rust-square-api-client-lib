//! Model for OrderServiceChargeType enum

use serde::{Deserialize, Serialize};

/// The type of the service charge.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderServiceChargeType {
    AutoGratuity,
    Custom,
}
