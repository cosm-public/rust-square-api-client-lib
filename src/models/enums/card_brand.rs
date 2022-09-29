//! Model for CardBrand enum

use serde::{Deserialize, Serialize};

/// Indicates a card's brand, such as `VISA` or `MASTERCARD`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardBrand {
    OtherBrand,
    Visa,
    Mastercard,
    AmericanExpress,
    Discover,
    DiscoverDiners,
    Jcb,
    ChinaUnionpay,
    SquareGiftCard,
    SquareCapitalCard,
    Interac,
    Eftpos,
    Felica,
    Ebt,
}
