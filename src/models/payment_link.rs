//! Model struct for PaymentRefund type

use serde::{Deserialize, Serialize};

/// Represents a payment link made using Square.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PaymentLink {
    /// The Square-assigned ID of the payment link.
    id: Option<String>,
    /// The Square-assigned version number, which is incremented each time an update
    /// is committed to the payment link.
    version: i32,
    // TODO: checkout_options: Option<CheckoutOptions>,
    /// The timestamp when the payment link was created, in RFC 3339 format.
    created_at: Option<String>,
    /// The optional description of the payment_link object.
    /// It is primarily for use by your application and is not used anywhere.
    description: Option<String>,
    /// The ID of the order associated with the payment link.
    order_id: Option<String>,
    /// An optional note. After Square processes the payment, this note is added to
    /// the resulting Payment.
    payment_note: Option<String>,
    // TODO: pre_populated_data: Option<PrePopulatedData>,
    /// The timestamp when the payment link was last updated, in RFC 3339 format.
    updated_at: Option<String>,
    /// The URL of the payment link.
    url: Option<String>,
}
