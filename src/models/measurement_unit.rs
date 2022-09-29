//! Model struct for MeasurementUnit type

use serde::{Deserialize, Serialize};

use super::{
    enums::{
        MeasurementUnitArea, MeasurementUnitGeneric, MeasurementUnitLength, MeasurementUnitTime,
        MeasurementUnitUnitType, MeasurementUnitVolume, MeasurementUnitWeight,
    },
    MeasurementUnitCustom,
};

/// Represents a unit of measurement to use with a quantity, such as ounces or inches.
///
/// Exactly one of the following fields are required: `custom_unit`, `area_unit`, `length_unit`,
/// `volume_unit`, and `weight_unit`.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct MeasurementUnit {
    /// A custom unit of measurement defined by the seller using the Point of Sale app or ad-hoc as
    /// an order line item.
    pub custom_unit: Option<MeasurementUnitCustom>,
    /// Represents a standard area unit.
    pub area_unit: Option<MeasurementUnitArea>,
    /// Represents a standard length unit.
    pub length_unit: Option<MeasurementUnitLength>,
    /// Represents a standard volume unit.
    pub volume_unit: Option<MeasurementUnitVolume>,
    /// Represents a standard unit of weight or mass.
    pub weight_unit: Option<MeasurementUnitWeight>,
    /// Reserved for API integrations that lack the ability to specify a real measurement unit.
    pub generic_unit: Option<MeasurementUnitGeneric>,
    /// Represents a standard unit of time.
    pub time_unit: Option<MeasurementUnitTime>,
    /// Represents the type of the measurement unit.
    pub r#type: Option<MeasurementUnitUnitType>,
}
