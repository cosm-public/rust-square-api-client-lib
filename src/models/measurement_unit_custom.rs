//! Model struct for MeasurementUnitCustom type

use serde::{Deserialize, Serialize};

/// The information needed to define a custom unit, provided by the seller.
#[derive(Clone, Debug, Default, Serialize, Eq, PartialEq, Deserialize)]
pub struct MeasurementUnitCustom {
    /// The name of the custom unit, for example "bushel".
    pub name: String,
    /// The abbreviation of the custom unit, such as "Bsh" (bushel). This appears in the cart for
    /// the Point of Sale app, and in reports.
    pub abbreviation: String,
}
