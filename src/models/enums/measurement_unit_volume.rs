//! Model for MeasurementUnitVolume enum

use serde::{Deserialize, Serialize};

/// The unit of volume used to measure a quantity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeasurementUnitVolume {
    /// The volume is measured in ounces.
    GenericFluidOunce,
    /// The volume is measured in shots.
    GenericShot,
    /// The volume is measured in cups.
    GenericCup,
    /// The volume is measured in pints.
    GenericPint,
    /// The volume is measured in quarts.
    GenericQuart,
    /// The volume is measured in gallons.
    GenericGallon,
    /// The volume is measured in cubic inches.
    ImperialCubicInch,
    /// The volume is measured in cubic feet.
    ImperialCubicFoot,
    /// The volume is measured in cubic yards.
    ImperialCubicYard,
    /// The volume is measured in metric milliliters.
    MetricMilliliter,
    /// The volume is measured in metric liters.
    MetricLiter,
}
