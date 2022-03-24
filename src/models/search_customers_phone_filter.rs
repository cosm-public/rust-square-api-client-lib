//! Model struct for SearchCustomersDateTimeFilter type

use crate::models::SearchCustomersTextFilter;
use serde::Serialize;
/// Filter for `Customers` objects based on whether their phone_numbers
///
/// A filter to select customers by their phone numbers visible to the seller.
/// This filter is case-insensitive.
///
/// **Important:** There are 2 kinds of searches available for phone_numbers: Exact and Fuzzy.
/// The details regarding it are available on the link below.
/// (https://developer.squareup.com/reference/square/customers/search-customers).
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct SearchCustomersPhoneFilter {
    /// The time range for filtering on the `created_at` timestamp. If you use this value, you must
    /// set the `sort_field` in the `CustomersSearchSort` object to `CREATED_AT`.
    pub phone_numbers: Option<SearchCustomersTextFilter>,
}
