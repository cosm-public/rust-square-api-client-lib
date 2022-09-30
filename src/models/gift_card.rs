//! Model struct for GiftCard type

use serde::{Deserialize, Serialize};

use super::{
    enums::{GiftCardGANSource, GiftCardStatus, GiftCardType},
    DateTime, Money,
};

/// Represents a Square gift card.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCard {
    pub id: Option<String>,
    pub r#type: GiftCardType,
    pub balance_money: Option<Money>,
    pub created_at: Option<DateTime>,
    pub customer_ids: Option<Vec<String>>,
    pub gan: Option<String>,
    pub gan_source: Option<GiftCardGANSource>,
    pub state: Option<GiftCardStatus>,
}
