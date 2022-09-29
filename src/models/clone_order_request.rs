//! Model struct for CloneOrderRequest type

use serde::Serialize;

/// This is a model struct for CloneOrderRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CloneOrderRequest {
    /// The ID of the order to clone.
    pub order_id: String,
    /// An optional order version for concurrency protection.
    ///
    /// If a version is provided, it must match the latest stored version of the order to clone. If
    /// a version is not provided, the API clones the latest version.
    pub version: Option<i32>,
    /// A value you specify that uniquely identifies this clone request.
    ///
    /// If you are unsure whether a particular order was cloned successfully, you can reattempt the
    /// call with the same idempotency key without worrying about creating duplicate cloned orders.
    /// The originally cloned order is returned.
    ///
    /// For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/basics/api101/idempotency).
    pub idempotency_key: Option<String>,
}
