//! Model struct for Coordinates type

use serde::{Deserialize, Serialize};

/// Latitude and longitude coordinates.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Coordinates {
    /// The latitude of the coordinate expressed in degrees.
    pub latitude: f64,
    /// The longitude of the coordinate expressed in degrees.
    pub longitude: f64,
}
