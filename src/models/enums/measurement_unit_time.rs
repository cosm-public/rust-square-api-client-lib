//! Model for MeasurementUnitTime enum

use serde::{Deserialize, Serialize};

/// Unit of time used to measure a quantity (a duration).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeasurementUnitTime {
    /// The time is measured in milliseconds.
    GenericMillisecond,
    /// The time is measured in seconds.
    GenericSecond,
    /// The time is measured in minutes.
    GenericMinute,
    /// The time is measured in hours.
    GenericHour,
    /// The time is measured in days.
    GenericDay,
}
