//! Model for PaymentSourceType enum

use serde::{Deserialize, Serialize};

/// Sources from which [Payment]s can be taken.
///
/// For information about these payment source types, see [Take
/// Payments](https://developer.squareup.com/docs/payments-api/take-payments).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentSourceType {
    /// A seller can charge a card (a credit/debit card or Square gift card). For more information,
    /// see [Take Card
    /// Payments](https://developer.squareup.com/docs/payments-api/take-payments/card-payments).
    Card,
    /// For more information, see [ACH Bank Transfer
    /// Payment](https://developer.squareup.com/docs/payments-api/take-payments/ach-payments).
    BankAccount,
    /// A seller can receive payments via Apple or Google Wallet.
    Wallet,
    /// For more information, see [Afterpay
    /// Payments](https://developer.squareup.com/docs/payments-api/take-payments/afterpay-payments).
    BuyNowPayLater,
    /// A seller can receive cash payments from a buyer and use `CreatePayment` to record the
    /// payment. For more information, see [Take Cash
    /// Payments](https://developer.squareup.com/docs/payments-api/take-payments/cash-payments).
    Cash,
    /// External payments refer to any payments processed by external entities (not by Square or the
    /// seller). For example, a buyer places a food order through a courier service. The courier
    /// service collects the payment and pays the seller. The seller can record these payments using
    /// `CreatePayment`. This does not change the seller's Square Balance. For more information, see
    /// [Take External
    /// Payments](https://developer.squareup.com/docs/payments-api/take-payments/external-payments).
    External,
}
