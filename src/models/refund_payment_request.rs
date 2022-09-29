//! Model struct for RefundPaymentRequest type

use serde::Serialize;

use super::Money;

/// This is a model struct for RefundPaymentRequest type
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct RefundPaymentRequest {
    /// A unique string that identifies this `RefundPayment` request. The key can be any valid
    /// string but must be unique for every `RefundPayment` request.
    ///
    /// Keys are limited to a max of 45 characters - however, the number of allowed characters might
    /// be less than 45, if multi-byte characters are used.
    ///
    /// For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/working-with-apis/idempotency).
    pub idempotency_key: String,
    /// The amount of money to refund.
    ///
    /// This amount cannot be more than the `total_money` value of the payment minus the total
    /// amount of all previously completed refunds for this payment.
    ///
    /// This amount must be specified in the smallest denomination of the applicable currency (for
    /// example, US dollar amounts are specified in cents). For more information, see [Working with
    /// Monetary
    /// Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts).
    ///
    /// The currency code must match the currency associated with the business that is charging the card.
    pub amount_money: Money,
    /// The amount of money the developer contributes to help cover the refunded amount. This amount
    /// is specified in the smallest denomination of the applicable currency (for example, US dollar
    /// amounts are specified in cents).
    ///
    /// The value cannot be more than the `amount_money`.
    ///
    /// You can specify this parameter in a refund request only if the same parameter was also
    /// included when taking the payment. This is part of the application fee scenario the API
    /// supports. For more information, see [Take Payments and Collect
    /// Fees](https://developer.squareup.com/docs/payments-api/take-payments-and-collect-fees).
    ///
    /// To set this field, `PAYMENTS_WRITE_ADDITIONAL_RECIPIENTS` OAuth permission is required. For
    /// more information, see
    /// [Permissions](https://developer.squareup.com/docs/payments-api/take-payments-and-collect-fees#permissions).
    pub app_fee_money: Option<Money>,
    /// The unique ID of the payment being refunded. Must be provided and non-empty.
    pub payment_id: String,
    /// A description of the reason for the refund.
    ///
    /// Max Length 192
    pub reason: Option<String>,
    /// Used for optimistic concurrency. This opaque token identifies the current `Payment` version
    /// that the caller expects. If the server has a different version of the Payment, the update
    /// fails and a response with a VERSION_MISMATCH error is returned. If the versions match, or
    /// the field is not provided, the refund proceeds as normal.
    pub payment_version_token: Option<String>,
    /// An optional [TeamMember] ID to associate with this refund.
    pub team_member_id: Option<String>,
}
