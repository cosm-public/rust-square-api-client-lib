//! Model struct for SearchCustomersDateTimeFilter type

use serde::Serialize;

use super::TimeRange;

/// Filter for `Customer` objects based on whether their `CREATED_AT`
/// timestamps fall within a specified time range.
///
/// You can specify the time range and which timestamp to filter for. You can filter for only one
/// time range at a time.
///
/// For each time range, the start time and end time are inclusive. If the end time is absent, it
/// defaults to the time of the first request for the cursor.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchCustomersDateTimeFilter {
    /// The time range for filtering on the `created_at` timestamp. If you use this value, you must
    /// set the `sort_field` in the `CustomersSearchSort` object to `CREATED_AT`.
    pub created_at: Option<TimeRange>,
}
