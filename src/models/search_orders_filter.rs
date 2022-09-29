//! Model struct for SearchOrdersFilter type

use serde::Serialize;

use super::{
    SearchOrdersCustomerFilter, SearchOrdersDateTimeFilter, SearchOrdersFulfillmentFilter,
    SearchOrdersSourceFilter, SearchOrdersStateFilter,
};

/// Filtering criteria to use for a `SearchOrders` request.
///
/// Multiple filters are ANDed together.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchOrdersFilter {
    /// Filter by [OrderState]
    pub state_filter: Option<SearchOrdersStateFilter>,
    /// Filter for results within a time range.
    ///
    /// **Important:** If you filter for orders by time range, you must set `SearchOrdersSort` to
    /// sort by the same field. [Learn more about filtering orders by time
    /// range](https://developer.squareup.com/docs/orders-api/manage-orders#important-note-on-filtering-orders-by-time-range).
    pub date_time_filter: Option<SearchOrdersDateTimeFilter>,
    /// Filter by the fulfillment type or state.
    pub fulfillment_filter: Option<SearchOrdersFulfillmentFilter>,
    /// Filter by the source of the order.
    pub source_filter: Option<SearchOrdersSourceFilter>,
    /// Filter by customers associated with the order.
    pub customer_filter: Option<SearchOrdersCustomerFilter>,
}
