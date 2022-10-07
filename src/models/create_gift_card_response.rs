//! Model struct for CreateGiftCardResponse type

use serde::Deserialize;

use super::{errors::Error, GiftCard};

/// This is a model struct for CreateGiftCardResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct CreateGiftCardResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The new gift card.
    pub gift_card: Option<GiftCard>,
}
