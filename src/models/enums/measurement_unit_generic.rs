//! Model for MeasurementUnitGeneric enum

use serde::{Deserialize, Serialize};

/// Reserved for API integrations that lack the ability to specify a real measurement unit.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeasurementUnitGeneric {
    /// The generic unit.
    Unit,
}
