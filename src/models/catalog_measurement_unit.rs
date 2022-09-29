//! Model struct for CatalogMeasurementUnit type.

use serde::{Deserialize, Serialize};

use super::MeasurementUnit;

/// Represents the unit used to measure a `CatalogItemVariation` and specifies the precision for
/// decimal quantities.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogMeasurementUnit {
    /// Indicates the unit used to measure the quantity of a catalog item variation.
    pub measurement_unit: Option<MeasurementUnit>,
    /// An integer between 0 and 5 that represents the maximum number of positions allowed after the
    /// decimal in quantities measured with this unit. For example:
    ///
    /// * if the precision is 0, the quantity can be 1, 2, 3, etc.
    /// * if the precision is 1, the quantity can be 0.1, 0.2, etc.
    /// * if the precision is 2, the quantity can be 0.01, 0.12, etc.
    /// Default: 3
    pub precision: Option<i32>,
}
