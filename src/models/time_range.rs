//! Model struct for TimeRange type

use serde::Serialize;

use super::DateTime;

/// Represents a generic time range.
///
/// Time ranges are customized to be inclusive or exclusive based on the needs of a particular
/// endpoint. Refer to the relevant endpoint-specific documentation to determine how time ranges are
/// handled.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct TimeRange {
    /// A datetime value indicating when the time range starts.
    pub start_at: Option<DateTime>,
    /// A datetime value indicating when the time range ends.
    pub end_at: Option<DateTime>,
}
