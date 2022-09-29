//! Model struct for PayOrderRequest type

use serde::Serialize;

/// This is a model struct for PayOrderRequest type
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct PayOrderRequest {
    /// A value you specify that uniquely identifies this request among requests you have sent. If
    /// you are unsure whether a particular payment request was completed successfully, you can
    /// reattempt it with the same idempotency key without worrying about duplicate payments.
    ///
    /// For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/basics/api101/idempotency).
    pub idempotency_key: String,
    /// The version of the order being paid. If not supplied, the latest version will be paid.
    pub order_version: Option<i32>,
    /// The IDs of the [Payment]s to collect. The payment total must match the order total.
    pub payment_ids: Option<Vec<String>>,
}
