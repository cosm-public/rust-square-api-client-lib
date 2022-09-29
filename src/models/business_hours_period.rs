//! Model struct for BusinessHoursPeriod type

use serde::{Deserialize, Serialize};

use super::enums::DayOfWeek;

/// The hours of operation for a location.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BusinessHoursPeriod {
    /// The day of the week for this time period.
    pub day_of_week: Option<DayOfWeek>,
    /// The end time of a business hours period, specified in local time using partial-time RFC 3339
    /// format. For example, `21:00:00` for a period ending at 9:00 in the evening. Note that the
    /// seconds value is always :00, but it is appended for conformance to the RFC.
    pub end_local_time: Option<String>,
    /// The start time of a business hours period, specified in local time using partial-time RFC
    /// 3339 format. For example, `21:00:00` for a period ending at 9:00 in the evening. Note that
    /// the seconds value is always :00, but it is appended for conformance to the RFC.
    pub start_local_time: Option<String>,
}
