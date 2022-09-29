//! Model for SearchOrdersSortField enum

use serde::{Deserialize, Serialize};

/// Reserved for API integrations that lack the ability to specify a real measurement unit.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchOrdersSortField {
    /// The time when the order was created. If you are also filtering for a time range in this
    /// query, you must set the `CREATED_AT` field in your `DateTimeFilter`.
    CreatedAt,
    /// The time when the order last updated. If you are also filtering for a time range in this
    /// query, you must set the `UPDATED_AT` field in your `DateTimeFilter`.
    UpdatedAt,
    /// The time when the order was closed. If you use this value, you must also set a `StateFilter`
    /// with closed states. If you are also filtering for a time range in this query, you must set
    /// the `CLOSED_AT` field in your `DateTimeFilter`.
    ClosedAt,
}
