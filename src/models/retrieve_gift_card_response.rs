//! Response struct for Retrieve Gift Card API

use serde::Deserialize;

use super::{errors::Error, GiftCard};

/// This is a model struct for RetrieveGiftCardResponse type
#[derive(Clone, Debug, Deserialize, Default, Eq, PartialEq)]
pub struct RetrieveGiftCardResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The gift card retrieved.
    pub gift_card: Option<GiftCard>,
}
