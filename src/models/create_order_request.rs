//! Model struct for CreateOrderRequest type

use serde::Serialize;

use super::Order;

/// This is a model struct for CreateOrderRequest type
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CreateOrderRequest {
    /// The order to create. If this field is set, the only other top-level field that can be set is
    /// the `idempotency_key`.
    pub order: Option<Order>,
    /// A value you specify that uniquely identifies this order among orders you have created.
    ///
    /// If you are unsure whether a particular order was created successfully, you can try it again
    /// with the same idempotency key without worrying about creating duplicate orders.
    ///
    /// For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/basics/api101/idempotency).
    pub idempotency_key: Option<String>,
}
