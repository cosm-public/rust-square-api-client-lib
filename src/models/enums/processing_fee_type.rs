//! Model for ProcessingFeeType enum.

use serde::{Deserialize, Serialize};

/// The type of fee assessed or adjusted.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProcessingFeeType {
    Initial,
    Adjustment,
}
