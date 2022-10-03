//! Model struct for GiftCard type

use serde::{Deserialize, Serialize};

use super::{
    enums::{GiftCardGANSource, GiftCardStatus, GiftCardType},
    DateTime, Money,
};

/// Represents a Square gift card.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCard {
    /// **Read only** The Square-assigned ID of the gift card.
    pub id: Option<String>,
    /// The gift card type.
    pub r#type: GiftCardType,
    /// **Read only** The current gift card balance. This balance is always greater than or equal
    /// to zero.
    pub balance_money: Option<Money>,
    /// **Read only** The timestamp when the gift card was created, in RFC 3339 format. In the case
    /// of a digital gift card, it is the time when you create a card (using the Square Point of
    /// Sale application, Seller Dashboard, or Gift Cards API).
    ///
    /// In the case of a plastic gift card, it is the time when Square associates the card with the
    /// seller at the time of activation.
    pub created_at: Option<DateTime>,
    /// **Read only** The IDs of the [customer profiles](Customer) to whom this gift card is linked.
    pub customer_ids: Option<Vec<String>>,
    /// The gift card account number (GAN). Buyers can use the GAN to make purchases or check the
    /// gift card balance.
    pub gan: Option<String>,
    /// The source that generated the gift card account number (GAN). The default value is `SQUARE`.
    pub gan_source: Option<GiftCardGANSource>,
    /// **Read only** The current gift card state.
    pub state: Option<GiftCardStatus>,
}
