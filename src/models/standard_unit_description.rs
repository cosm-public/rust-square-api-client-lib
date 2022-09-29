//! Model struct for StandardUnitDescription type.

use serde::Deserialize;

use super::MeasurementUnit;

/// Contains the name and abbreviation for standard measurement unit.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct StandardUnitDescription {
    /// Identifies the measurement unit being described.
    pub unit: Option<MeasurementUnit>,
    /// UI display name of the measurement unit. For example, 'Pound'.
    pub name: Option<String>,
    /// UI display abbreviation for the measurement unit. For example, 'lb'.
    pub abbreviation: Option<String>,
}
