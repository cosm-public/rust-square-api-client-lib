//! Model for SearchOrdersSortField enum

use serde::{Deserialize, Serialize};

/// Reserved for API integrations that lack the ability to specify a real measurement unit.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchCustomersSortField {
    /// The time when the order was created. If you are also filtering for a time range in this
    /// query, you must set the `CREATED_AT` field in your `DateTimeFilter`.
    CreatedAt,
}
