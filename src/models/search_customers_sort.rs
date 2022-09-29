//! Model struct for SearchCustomersSort type

use crate::models::enums::{SortCustomersField, SortOrder};
use serde::Serialize;
/// Sorting criteria for a `SearchCustomers` request.
///
/// Results can only be sorted by a timestamp field.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchCustomersSort {
    /// The field to sort by.
    ///
    /// **Important:** When using a [DateTimeFilter](SearchCustomersDateTimeFilter), `sort_field` must
    /// match the timestamp field that the `DateTimeFilter` uses to filter. For example, if you set
    /// your `sort_field` to `CREATED_AT` and you use a `DateTimeFilter`, your `DateTimeFilter` must
    /// filter for customers by their `CREATED_AT` date. If this field does not match the timestamp
    /// field in `DateTimeFilter`, `SearchCustomers` returns an error.
    ///
    /// Default: `CREATED_AT`.
    pub sort_field: Option<SortCustomersField>,
    /// The chronological order in which results are returned. Defaults to DESC.
    pub sort_order: Option<SortOrder>,
}
