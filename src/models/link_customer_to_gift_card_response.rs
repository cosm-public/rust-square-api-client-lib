//! The response body for the Link Customer To Gift Card API

use serde::Deserialize;

use super::{errors::Error, GiftCard};

/// This is a model struct for LinkCustomerToGiftCardResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct LinkCustomerToGiftCardResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The gift card with the ID of the linked customer listed in the `customer_ids` field.
    pub gift_card: Option<GiftCard>,
}
