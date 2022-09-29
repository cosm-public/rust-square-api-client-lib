//! Model struct for UpdateOrderRequest type

use serde::Serialize;

use super::Order;

/// This is a model struct for UpdateOrderRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateOrderRequest {
    /// The [sparse
    /// order](https://developer.squareup.com/docs/orders-api/manage-orders#sparse-order-objects)
    /// containing only the fields to update and the version to which the update is being applied.
    pub order: Order,
    /// The [dot notation
    /// paths](https://developer.squareup.com/docs/orders-api/manage-orders#on-dot-notation) fields
    /// to clear. For example, `line_items[uid].note`. For more information, see [Deleting
    /// fields](https://developer.squareup.com/docs/orders-api/manage-orders#delete-fields).
    pub fields_to_clear: Option<Vec<String>>,
    /// A value you specify that uniquely identifies this update request.
    ///
    /// If you are unsure whether a particular update was applied to an order successfully, you can
    /// reattempt it with the same idempotency key without worrying about creating duplicate updates
    /// to the order. The latest order version is returned.
    ///
    /// For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/basics/api101/idempotency).
    pub idempotency_key: Option<String>,
}
