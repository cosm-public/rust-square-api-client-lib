//! Model for ErrorCategory enum

use serde::{Deserialize, Serialize};

/// Indicates which high-level category of error has occurred during a request to the Connect API.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCategory {
    /// An error occurred with the Connect API itself.
    ApiError,
    /// An authentication error occurred. Most commonly, the request had a missing, malformed, or
    /// otherwise invalid `Authorization` header.
    AuthenticationError,
    /// The request was invalid. Most commonly, a required parameter was missing, or a provided
    /// parameter had an invalid value.
    InvalidRequestError,
    /// Your application reached the Square API rate limit. You might receive this error if your
    /// application sends a high number of requests to Square APIs in a short period of time.
    ///
    /// Your application should monitor responses for `429 RATE_LIMITED` errors and use a retry
    /// mechanism with an [exponential backoff](https://en.wikipedia.org/wiki/Exponential_backoff)
    /// schedule to resend the requests at an increasingly slower rate. It is also a good practice
    /// to use a randomized delay (jitter) in your retry schedule.
    RateLimitError,
    /// An error occurred while processing a payment method. Most commonly, the details of the
    /// payment method were invalid (such as a card's CVV or expiration date).
    PaymentMethodError,
    /// An error occurred while attempting to process a refund.
    RefundError,
    /// An error occurred when checking a merchant subscription status
    MerchantSubscriptionError,
}
