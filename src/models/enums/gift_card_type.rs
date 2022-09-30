//! Model for GiftCardType enum

use serde::{Deserialize, Serialize};

/// Indicates the gift card type.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardType {
    /// A plastic gift card.
    Physical,
    /// A digital gift card.
    Digital,
}

impl Default for GiftCardType {
    fn default() -> Self {
        Self::Physical
    }
}
