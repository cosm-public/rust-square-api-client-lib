//! Model for MeasurementUnitLength enum

use serde::{Deserialize, Serialize};

/// The unit of length used to measure a quantity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeasurementUnitLength {
    /// The length is measured in inches.
    ImperialInch,
    /// The length is measured in feet.
    ImperialFoot,
    /// The length is measured in yards.
    ImperialYard,
    /// The length is measured in miles.
    ImperialMile,
    /// The length is measured in millimeters.
    MetricMillimeter,
    /// The length is measured in centimeters.
    MetricCentimeter,
    /// The length is measured in meters.
    MetricMeter,
    /// The length is measured in kilometers.
    MetricKilometer,
}
