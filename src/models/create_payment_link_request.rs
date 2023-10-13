//! Request struct for the Create Payment API

use serde::Serialize;

use super::{CheckoutOptions, Order, QuickPay};

/// This is a model class for CreatePaymentRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CreatePaymentLinkRequest {
    /// A unique string that identifies this [`CreatePaymentLinkRequest`]. Keys can be any valid string
    /// but must be unique for every [`CreatePaymentLinkRequest`].
    ///
    /// Note: The number of allowed characters might be less than the stated maximum, if multi-byte
    /// characters are used.
    ///
    /// For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/working-with-apis/idempotency).
    idempotency_key: Option<String>,

    /// A description of the payment link.
    ///
    /// You provide this optional description that is useful in your application context.
    /// It is not used anywhere.
    description: Option<String>,

    /// Describes an ad hoc item and price for which to generate a quick pay checkout link.
    ///
    /// For more information, see
    /// [Quick Pay Checkout](https://developer.squareup.com/docs/checkout-api/quick-pay-checkout).
    quick_pay: Option<QuickPay>,

    /// Describes the [`Order`] for which to create a checkout link.
    ///
    /// For more information, see
    /// [Square Order Checkout](https://developer.squareup.com/docs/checkout-api/square-order-checkout).
    order: Option<Order>,

    /// Describes optional fields to add to the resulting checkout page.
    ///
    /// For more information, see
    /// [Optional Checkout Configurations](https://developer.squareup.com/docs/checkout-api/optional-checkout-configurations).
    checkout_options: Option<CheckoutOptions>,

    /// Describes fields to prepopulate in the resulting checkout page.
    ///
    /// For more information, see
    /// [Prepopulate the shipping address](https://developer.squareup.com/docs/checkout-api/optional-checkout-configurations#prepopulate-the-shipping-address).
    // TODO: pre_populated_data: Option<PrePopulatedData>,

    /// A note for the payment.
    ///
    /// After processing the payment, Square adds this note to the resulting `Payment`.
    payment_note: Option<String>,
}
