//! Model struct for Subscription type

use serde::{Deserialize, Serialize};

use super::{
    enums::{SubscriptionStatus, Timezone},
    DateTime, Money, SubscriptionAction, SubscriptionSource,
};

/// Represents a subscription to a subscription plan by a subscriber.
///
/// For an overview of the `Subscription` type, see [Subscription
/// object](https://developer.squareup.com/docs/subscriptions-api/overview#subscription-object-overview).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Subscription {
    /// **Read only** The Square-assigned ID of the subscription.
    ///
    /// Max Length: 255
    pub id: Option<String>,
    /// **Read only** The ID of the location associated with the subscription.
    pub location_id: Option<String>,
    /// **Read only** The ID of the subscribed-to [subscription plan](CatalogSubscriptionPlan).
    pub plan_id: Option<String>,
    /// **Read only** The ID of the subscribing [customer](Customer) profile.
    pub customer_id: Option<String>,
    /// **Read only** The `YYYY-MM-DD`-formatted date (for example, 2013-01-15) to start the
    /// subscription.
    pub start_date: Option<String>,
    /// The `YYYY-MM-DD`-formatted date (for example, 2013-01-15) to cancel the subscription, when
    /// the subscription status changes to `CANCELED` and the subscription billing stops.
    ///
    /// If this field is not set, the subscription ends according its subscription plan.
    ///
    /// This field cannot be updated, other than being cleared.
    pub canceled_date: Option<String>,
    /// **Read only** The `YYYY-MM-DD-formatted` date up to when the subscriber is invoiced for the
    /// subscription.
    ///
    /// After the invoice is sent for a given billing period, this date will be the last day of the
    /// billing period. For example, suppose for the month of May a subscriber gets an invoice (or
    /// charged the card) on May 1. For the monthly billing scenario, this date is then set to May
    /// 31.
    pub charged_through_date: Option<String>,
    /// **Read only** The current status of the subscription.
    pub status: Option<SubscriptionStatus>,
    /// The tax amount applied when billing the subscription. The percentage is expressed in decimal
    /// form, using a `'.'` as the decimal separator and without a `'%'` sign. For example, a value
    /// of `7.5` corresponds to 7.5%.
    pub tax_percentage: Option<String>,
    /// **Read only** The IDs of the [invoices](Invoice) created for the subscription, listed in
    /// order when the invoices were created (newest invoices appear first).
    pub invoice_ids: Option<Vec<String>>,
    /// A custom price to apply for the subscription. If specified, it overrides the price
    /// configured by the subscription plan.
    pub price_override_money: Option<Money>,
    /// The version of the object. When updating an object, the version supplied must match the
    /// version in the database, otherwise the write will be rejected as conflicting.
    pub version: Option<i64>,
    /// Read only The timestamp when the subscription was created, in RFC 3339 format.
    pub create_at: Option<DateTime>,
    /// The ID of the [subscriber's card](Customer) used to charge for the subscription.
    pub card_id: Option<String>,
    /// **Read only** Timezone that will be used in date calculations for the subscription. Defaults
    /// to the timezone of the location based on `location_id`. Format: the IANA Timezone Database
    /// identifier for the location timezone (for example, `America/Los_Angeles`).
    pub timezone: Option<Timezone>,
    /// The origination details of the subscription.
    pub source: Option<SubscriptionSource>,
    /// The list of scheduled actions on this subscription. It is set only in the response from
    /// [RetrieveSubscription](https://developer.squareup.com/reference/square/subscriptions-api/retrieve-subscription)
    /// with the query parameter of `include=actions` or from
    /// [SearchSubscriptions](https://developer.squareup.com/reference/square/subscriptions-api/search-subscriptions)
    /// with the input parameter of `include:["actions"]`.
    pub actions: Option<Vec<SubscriptionAction>>,
}
