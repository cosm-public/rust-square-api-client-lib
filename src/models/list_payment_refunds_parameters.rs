//! Model struct for ListPaymentRefundsParameters (query parameters)

use super::{
    enums::{PaymentRefundStatus, PaymentSourceType, SortOrder},
    DateTime,
};

/// This is a model struct for ListPaymentRefundsParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListPaymentRefundsParameters {
    /// The timestamp for the beginning of the reporting period. Inclusive.
    ///
    /// Default: The current time minus one year.
    pub begin_time: Option<DateTime>,
    /// The timestamp for the end of the reporting period.
    ///
    /// Default: The current time.
    pub end_time: Option<DateTime>,
    /// The order in which results are listed.
    /// * `ASC` - Oldest to newest.
    /// * `DESC` - Newest to oldest (default).
    pub sort_order: Option<SortOrder>,
    /// A pagination cursor returned by a previous call to this endpoint. Provide this cursor to
    /// retrieve the next set of results for the original query.
    ///
    /// For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// Limit results to the location supplied. By default, results are returned for all locations
    /// associated with the seller.
    pub location_id: Option<String>,
    /// If provided, only refunds with the given status are returned.
    ///
    /// Default: If omitted, refunds are returned regardless of their status.
    pub status: Option<PaymentRefundStatus>,
    /// If provided, only returns refunds whose payments have the indicated source type. Current
    /// values include CARD, BANK_ACCOUNT, WALLET, CASH, and EXTERNAL. For information about these
    /// payment source types,
    /// see [Take Payments](https://developer.squareup.com/docs/payments-api/take-payments).
    ///
    /// Default: If omitted, refunds are returned regardless of the source type.
    pub source_type: Option<PaymentSourceType>,
    /// The maximum number of results to be returned in a single page. It is possible to receive
    /// fewer results than the specified limit on a given page.
    ///
    /// The default value of 100 is also the maximum allowed value. If the provided value is greater
    /// than 100, it is ignored and the default value is used instead.
    ///
    /// Default: `100`
    pub limit: Option<i32>,
}

impl ListPaymentRefundsParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListPaymentRefundsParameters> for String {
    fn from(list_payment_refunds_parameters: ListPaymentRefundsParameters) -> Self {
        list_payment_refunds_parameters.to_string()
    }
}

impl ToString for ListPaymentRefundsParameters {
    fn to_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(begin_time) = &self.begin_time {
            params.push(format!("begin_time={}", begin_time));
        }

        if let Some(end_time) = &self.end_time {
            params.push(format!("end_time={}", end_time));
        }

        if let Some(sort_order) = &self.sort_order {
            params.push(format!("sort_order={}", serde_json::to_string(sort_order).unwrap()));
        }

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(location_id) = &self.location_id {
            params.push(format!("location_id={}", location_id));
        }

        if let Some(status) = &self.status {
            params.push(format!("status={}", serde_json::to_string(status).unwrap()));
        }

        if let Some(source_type) = &self.source_type {
            params.push(format!("source_type={}", serde_json::to_string(source_type).unwrap()));
        }

        if let Some(limit) = &self.limit {
            params.push(format!("limit={}", limit));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}
