//! Request body struct for the Create Subscription API

use serde::Serialize;

use super::{enums::Timezone, Money, SubscriptionSource};

/// This is a model class for CreateSubscriptionRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CreateSubscriptionRequest {
    pub idempotency_key: Option<String>,
    pub location_id: String,
    pub plan_id: String,
    pub customer_id: String,
    pub start_date: Option<String>,
    pub canceled_date: Option<String>,
    pub tax_percentage: Option<String>,
    pub price_override_money: Option<Money>,
    pub card_id: Option<String>,
    pub timezone: Option<Timezone>,
    pub source: Option<SubscriptionSource>,
}
