//! Model for MeasurementUnitArea enum

use serde::{Deserialize, Serialize};

/// Unit of area used to measure a quantity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeasurementUnitArea {
    /// The area is measured in acres.
    ImperialAcre,
    /// The area is measured in square inches.
    ImperialSquareInch,
    /// The area is measured in square feet.
    ImperialSquareFoot,
    /// The area is measured in square yards.
    ImperialSquareYard,
    /// The area is measured in square miles.
    ImperialSquareMile,
    /// The area is measured in square centimeters.
    MetricSquareCentimeter,
    /// The area is measured in square meters.
    MetricSquareMeter,
    /// The area is measured in square kilometers.
    MetricSquareKilometer,
}
