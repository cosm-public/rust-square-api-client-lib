//! Model for ErrorCode enum

use serde::{Deserialize, Serialize};

/// Indicates which high-level category of error has occurred during a request to the Connect API.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    /// A general server error occurred.
    InternalServerError,
    /// A general authorization error occurred.
    Unauthorized,
    /// The provided access token has expired.
    AccessTokenExpired,
    /// The provided access token has been revoked.
    AccessTokenRevoked,
    /// The provided client has been disabled.
    ClientDisabled,
    /// A general access error occurred.
    Forbidden,
    /// The provided access token does not have permission to execute the requested action.
    InsufficientScopes,
    /// The calling application was disabled.
    ApplicationDisabled,
    /// The calling application was created prior to 2016-03-30 and is not compatible with v2 Square
    /// API calls.
    V1Application,
    /// The calling application is using an access token created prior to 2016-03-30 and is not
    /// compatible with v2 Square API calls.
    V1AccessToken,
    /// The location provided in the API call is not enabled for credit card processing.
    CardProcessingNotEnabled,
    /// A required subscription was not found for the merchant
    MerchantSubscriptionNotFound,
    /// A general error occurred with the request.
    BadRequest,
    /// The request is missing a required path, query, or body parameter.
    MissingRequiredParameter,
    /// The value provided in the request is the wrong type. For example, a string instead of an
    /// integer.
    IncorrectType,
    /// Formatting for the provided time value is incorrect.
    InvalidTime,
    /// The time range provided in the request is invalid. For example, the end time is before the
    /// start time.
    InvalidTimeRange,
    /// The provided value is invalid. For example, including `%` in a phone number.
    InvalidValue,
    /// The pagination cursor included in the request is invalid.
    InvalidCursor,
    /// The query parameters provided is invalid for the requested endpoint.
    UnknownQueryParameter,
    /// One or more of the request parameters conflict with each other.
    ConflictingParameters,
    /// The request body is not a JSON object.
    ExpectedJsonBody,
    /// The provided sort order is not a valid key. Currently, sort order must be `ASC` or `DESC`.
    InvalidSortOrder,
    /// The provided value does not match an expected regular expression.
    ValueRegexMismatch,
    /// The provided string value is shorter than the minimum length allowed.
    ValueTooShort,
    /// The provided string value is longer than the maximum length allowed.
    ValueTooLong,
    /// The provided value is less than the supported minimum.
    ValueTooLow,
    /// The provided value is greater than the supported maximum.
    ValueTooHigh,
    /// The provided value has a default (empty) value such as a blank string.
    ValueEmpty,
    /// The provided array has too many elements.
    ArrayLengthTooLong,
    /// The provided array has too few elements.
    ArrayLengthTooShort,
    /// The provided array is empty.
    ArrayEmpty,
    /// The endpoint expected the provided value to be a boolean.
    ExpectedBoolean,
    /// The endpoint expected the provided value to be an integer.
    ExpectedInteger,
    /// The endpoint expected the provided value to be a float.
    ExpectedFloat,
    /// The endpoint expected the provided value to be a string.
    ExpectedString,
    /// The endpoint expected the provided value to be a JSON object.
    ExpectedObject,
    /// The endpoint expected the provided value to be an array or list.
    ExpectedArray,
    /// The endpoint expected the provided value to be a map or associative array.
    ExpectedMap,
    /// The endpoint expected the provided value to be an array encoded in base64.
    ExpectedBase64EncodedByteArray,
    /// One or more objects in the array does not match the array type.
    InvalidArrayValue,
    /// The provided static string is not valid for the field.
    InvalidEnumValue,
    /// Invalid content type header.
    InvalidContentType,
    /// Only relevant for applications created prior to 2016-03-30. Indicates there was an error
    /// while parsing form values.
    InvalidFormValue,
    /// The provided customer id can't be found in the merchant's customers list.
    CustomerNotFound,
    /// A general error occurred.
    OneInstrumentExpected,
    /// A general error occurred.
    NoFieldsSet,
    /// Too many entries in the map field.
    TooManyMapEntries,
    /// The length of one of the provided keys in the map is too short.
    MapKeyLengthTooShort,
    /// The length of one of the provided keys in the map is too long.
    MapKeyLengthTooLong,
    /// The provided customer does not have a recorded name.
    CustomerMissingName,
    /// The provided customer does not have a recorded email.
    CustomerMissingEmail,
    /// The subscription cannot be paused longer than the duration of the current phase.
    InvalidPauseLength,
    /// The subscription cannot be paused/resumed on the given date.
    InvalidDate,
    /// The card issuer declined the request because the card is expired.
    CardExpired,
    /// The expiration date for the payment card is invalid. For example, it indicates a date in the
    /// past.
    InvalidExpiration,
    /// The expiration year for the payment card is invalid. For example, it indicates a year in the
    /// past or contains invalid characters.
    InvalidExpirationYear,
    /// The expiration date for the payment card is invalid. For example, it contains invalid
    /// characters.
    InvalidExpirationDate,
    /// The credit card provided is not from a supported issuer.
    UnsupportedCardBrand,
    /// The entry method for the credit card (swipe, dip, tap) is not supported.
    UnsupportedEntryMethod,
    /// The encrypted card information is invalid.
    InvalidEncryptedCard,
    /// The credit card cannot be validated based on the provided details.
    InvalidCard,
    /// Square received a decline without any additional information. If the payment information
    /// seems correct, the buyer can contact their issuer to ask for more information.
    GenericDecline,
    /// The card issuer declined the request because the CVV value is invalid.
    CvvFailure,
    /// The card issuer declined the request because the postal code is invalid.
    AddressVerificationFailure,
    /// The issuer was not able to locate the account on record.
    InvalidAccount,
    /// The currency associated with the payment is not valid for the provided funding source. For
    /// example, a gift card funded in USD cannot be used to process payments in GBP.
    CurrencyMismatch,
    /// The funding source has insufficient funds to cover the payment.
    InsufficientFunds,
    /// The Square account does not have the permissions to accept this payment. For example, Square
    /// may limit which merchants are allowed to receive gift card payments.
    InsufficientPermissions,
    /// The card issuer has declined the transaction due to restrictions on where the card can be
    /// used. For example, a gift card is limited to a single merchant.
    CardholderInsufficientPermissions,
    /// The Square account cannot take payments in the specified region. A Square account can take
    /// payments only from the region where the account was created.
    InvalidLocation,
    /// The card issuer has determined the payment amount is either too high or too low. The API
    /// returns the error code mostly for credit cards (for example, the card reached the credit
    /// limit). However, sometimes the issuer bank can indicate the error for debit or prepaid cards
    /// (for example, card has insufficient funds).
    TransactionLimit,
    /// The card issuer declined the request because the issuer requires voice authorization from
    /// the cardholder.
    VoiceFailure,
    /// The specified card number is invalid. For example, it is of incorrect length or is
    /// incorrectly formatted.
    PanFailure,
    /// The card expiration date is either invalid or indicates that the card is expired.
    ExpirationFailure,
    /// The card is not supported either in the geographic region or by the [merchant category
    /// code](https://developer.squareup.com/docs/locations-api#initialize-a-merchant-category-code)
    /// (MCC).
    CardNotSupported,
    /// The card issuer declined the request because the PIN is invalid.
    InvalidPin,
    /// The payment is missing a required PIN.
    MissingPin,
    /// The payment is missing a required `ACCOUNT_TYPE` parameter.
    MissingAccountType,
    /// The postal code is incorrectly formatted.
    InvalidPostalCode,
    /// The `app_fee_money` on a payment is too high.
    InvalidFees,
    /// The card must be swiped, tapped, or dipped. Payments attempted by manually entering the card
    /// number are declined.
    ManuallyEnteredPaymentNotSupported,
    /// Square declined the request because the payment amount exceeded the processing limit for
    /// this merchant.
    PaymentLimitExceeded,
    /// When a Gift Card is a payment source, you can allow taking a partial payment by adding the
    /// `accept_partial_authorization` parameter in the request. However, taking such a partial
    /// payment does not work if your request also includes `tip_money`, `app_fee_money`, or both.
    /// Square declines such payments and returns the `GIFT_CARD_AVAILABLE_AMOUNT` error. For more
    /// information, see [CreatePayment errors (additional
    /// information)](https://developer.squareup.com/docs/payments-api/error-codes#createpayment-errors-additional-information).
    GiftCardAvailableAmount,
    /// The account provided cannot carry out transactions.
    AccountUnusable,
    /// Bank account rejected or was not authorized for the payment.
    BuyerRefusedPayment,
    /// The application tried to update a delayed-capture payment that has expired.
    DelayedTransactionExpired,
    /// The application tried to cancel a delayed-capture payment that was already cancelled.
    DelayedTransactionCanceled,
    /// The application tried to capture a delayed-capture payment that was already captured.
    DelayedTransactionCaptured,
    /// The application tried to update a delayed-capture payment that failed.
    DelayedTransactionFailed,
    /// The provided card token (nonce) has expired.
    CardTokenExpired,
    /// The provided card token (nonce) was already used to process payment.
    CardTokenUsed,
    /// The requested payment amount is too high for the provided payment source.
    AmountTooHigh,
    /// The API request references an unsupported instrument type.
    UnsupportedInstrumentType,
    /// The requested refund amount exceeds the amount available to refund.
    RefundAmountInvalid,
    /// The payment already has a pending refund.
    RefundAlreadyPending,
    /// The payment is not refundable. For example, a previous refund has already been rejected and
    /// no new refunds can be accepted.
    PaymentNotRefundable,
    /// Request failed - The card issuer declined the refund.
    RefundDeclined,
    /// Generic error - the provided card data is invalid.
    InvalidCardData,
    /// The provided source id was already used to create a card.
    SourceUsed,
    /// The provided source id has expired.
    SourceExpired,
    /// The referenced loyalty program reward tier is not supported. This could happen if the reward
    /// tier created in a first party application is incompatible with the Loyalty API.
    UnsupportedLoyaltyRewardTier,
    /// Generic error - the given location does not matching what is expected.
    LocationMismatch,
    /// The provided idempotency key has already been used.
    IdempotencyKeyReused,
    /// General error - the value provided was unexpected.
    UnexpectedValue,
    /// The API request is not supported in sandbox.
    SandboxNotSupported,
    /// The provided email address is invalid.
    InvalidEmailAddress,
    /// The provided phone number is invalid.
    InvalidPhoneNumber,
    /// The provided checkout URL has expired.
    CheckoutExpired,
    /// Bad certificate.
    BadCertificate,
    /// The provided Square-Version is incorrectly formatted.
    InvalidSquareVersionFormat,
    /// The provided Square-Version is incompatible with the requested action.
    ApiVersionIncompatible,
    /// The card was declined.
    CardDeclined,
    /// The CVV could not be verified.
    VerifyCvvFailure,
    /// The AVS could not be verified.
    VerifyAvsFailure,
    /// The payment card was declined with a request for the card holder to call the issuer.
    CardDeclinedCallIssure,
    /// The payment card was declined with a request for additional verification.
    CardDeclinedVerificationRequired,
    /// The card expiration date is either missing or incorrectly formatted.
    BadExpiration,
    /// The card issuer requires that the card be read using a chip reader.
    ChipInsertionRequired,
    /// The card has exhausted its available pin entry retries set by the card issuer. Resolving the
    /// error typically requires the card holder to contact the card issuer.
    AllowablePinTriesExceeded,
    /// The card issuer declined the refund.
    ReservationDeclined,
    /// Not Found - a general error occurred.
    NotFound,
    /// Square could not find the associated Apple Pay certificate.
    ApplePaymentProcessingCertificateHashNotFound,
    /// Method Not Allowed - a general error occurred.
    MethodNotAllowed,
    /// Not Acceptable - a general error occurred.
    NotAcceptable,
    /// Request Timeout - a general error occurred.
    RequestTimeout,
    /// Conflict - a general error occurred.
    Conflict,
    /// The target resource is no longer available and this condition is likely to be permanent.
    Gone,
    /// Request Entity Too Large - a general error occurred.
    RequestEntityTooLarge,
    /// Unsupported Media Type - a general error occurred.
    UnsupportedMediaType,
    /// Unprocessable Entity - a general error occurred.
    UnprocessableEntity,
    /// Rate Limited - a general error occurred.
    RateLimited,
    /// Not Implemented - a general error occurred.
    NotImplemented,
    /// Bad Gateway - a general error occurred.
    BadGateway,
    /// Service Unavailable - a general error occurred.
    ServiceUnavailable,
    /// A temporary internal error occurred. You can safely retry your call using the same
    /// idempotency key.
    TemporaryError,
    /// Gateway Timeout - a general error occurred.
    GatewayTimeout,
}
