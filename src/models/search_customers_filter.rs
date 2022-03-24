//! Model struct for SearchCustomersFilter type

use serde::Serialize;

use super::{
    SearchCustomersDateTimeFilter, SearchCustomersEmailAddressFilter, SearchCustomersPhoneFilter,
};

/// Filtering criteria to use for a `SearchCustomers` request.
///
/// Multiple filters are ANDed together.
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct SearchCustomersFilter {
    /// Filter by [ email addresses]
    pub enail_address_filter: Option<SearchCustomersEmailAddressFilter>,
    /// Filter for results within a time range.
    ///
    /// **Important:** If you filter for customers by time range, you must set `SearchCustomersSort` to
    /// sort by the same field. [Learn more about filtering customers by time
    /// range](https://developer.squareup.com/docs/customers-api/manage-customers#important-note-on-filtering-customers-by-time-range).
    pub date_time_filter: Option<SearchCustomersDateTimeFilter>,
    /// Filter by the phone number.
    pub phone_number_filter: Option<SearchCustomersPhoneFilter>,
}
