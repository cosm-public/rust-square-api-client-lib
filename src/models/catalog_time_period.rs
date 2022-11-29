//! Model struct for CatalogTimePeriod type.

use serde::{Deserialize, Serialize};

/// Represents a time period - either a single period or a repeating period.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogTimePeriod {
    /// An iCalendar (RFC 5545) [event](https://tools.ietf.org/html/rfc5545#section-3.6.1), which
    /// specifies the name, timing, duration and recurrence of this time period.
    ///
    /// Example:
    /// ```
    /// use square_api_client::models::CatalogTimePeriod;
    ///
    /// CatalogTimePeriod {
    ///     event: Some(String::from("DTSTART:20190707T180000\nDURATION:P2H\nRRULE:FREQ=WEEKLY;BYDAY=MO,WE,FR")),
    /// };
    /// ```
    /// Only `SUMMARY`, `DTSTART`, `DURATION` and `RRULE` fields are supported. `DTSTART` must be in
    /// local (unzoned) time format. Note that while `BEGIN:VEVENT` and `END:VEVENT` is not required
    /// in the request. The response will always include them.
    pub event: Option<String>,
}
