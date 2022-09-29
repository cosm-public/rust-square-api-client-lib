//! Model for MeasurementUnitWeight enum

use serde::{Deserialize, Serialize};

/// Unit of weight used to measure a quantity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeasurementUnitWeight {
    /// The weight is measured in ounces.
    ImperialWeightOunce,
    /// The weight is measured in pounds.
    ImperialPound,
    /// The weight is measured in stones.
    ImperialStone,
    /// The weight is measured in milligrams.
    MetricMilligram,
    /// The weight is measured in grams.
    MetricGram,
    /// The weight is measured in kilograms.
    MetricKilogram,
}
