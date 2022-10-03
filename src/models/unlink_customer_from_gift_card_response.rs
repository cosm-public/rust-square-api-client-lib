//! The response body for the Unlink Customer From Gift Card API

use serde::Deserialize;

use super::{errors::Error, GiftCard};

/// This is a model struct for UnlinkCustomerFromGiftCardResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct UnlinkCustomerFromGiftCardResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The gift card with the ID of the unlinked customer removed from the `customer_ids` field.
    /// If no other customers are linked, the `customer_ids` field is also removed.
    pub gift_card: Option<GiftCard>,
}
