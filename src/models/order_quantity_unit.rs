//! Model struct for OrderQuantityUnit type

use serde::{Deserialize, Serialize};

use super::MeasurementUnit;

/// Contains the measurement unit for a quantity and a precision that specifies the number of digits
/// after the decimal point for decimal quantities.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderQuantityUnit {
    /// A [MeasurementUnit] that represents the unit of measure for the quantity.
    pub measurement_unit: Option<MeasurementUnit>,
    /// For non-integer quantities, represents the number of digits after the decimal point that are
    /// recorded for this quantity.
    ///
    /// For example, a precision of 1 allows quantities such as "1.0" and "1.1", but not "1.01".
    ///
    /// Min: 0. Max: 5.
    pub precision: Option<i32>,
    /// The catalog object ID referencing the [CatalogMeasurementUnit].
    ///
    /// This field is set when this is a catalog-backed measurement unit.
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this measurement unit references.
    ///
    /// This field is set when this is a catalog-backed measurement unit.
    pub catalog_version: Option<i64>,
}
