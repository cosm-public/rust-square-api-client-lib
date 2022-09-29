//! Model struct for SearchCustomersFilter type

use serde::Serialize;

use super::{
    SearchCustomerCreationSourceFilter, SearchCustomersDateTimeFilter, SearchCustomersTextFilter,
};

/// Filtering criteria to use for a `SearchCustomers` request.
///
/// Multiple filters are ANDed together.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchCustomersFilter {
    /// Filter by [ email addresses]
    pub email_address: Option<SearchCustomersTextFilter>,
    /// Filter for results within a time range.
    ///
    /// **Important:** If you filter for customers by time range, you must set `SearchCustomersSort` to
    /// sort by the same field. [Learn more about filtering customers by time
    /// range](https://developer.squareup.com/docs/customers-api/manage-customers#important-note-on-filtering-customers-by-time-range).
    pub created_at: Option<SearchCustomersDateTimeFilter>,
    /// Filter by the phone number.
    pub phone_number: Option<SearchCustomersTextFilter>,
    /// Filter by the the secondary id that third party systems can inject when creating the customer.
    pub reference_id: Option<SearchCustomersTextFilter>,
    ///The creation source filter.
    /// If one or more creation sources are set, customer profiles are included in,
    ///or excluded from, the result if they match at least one of the filter criteria.
    pub creation_source: Option<SearchCustomerCreationSourceFilter>,
    /// Similar to created_at, we can also fulter with a time range of updated_at property_names
    pub updated_at: Option<SearchCustomersDateTimeFilter>,
}
