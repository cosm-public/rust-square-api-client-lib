//! Response struct for Retrieve Gift Card From GAN API

use serde::Deserialize;

use super::{errors::Error, GiftCard};

/// This is a model struct for RetrieveGiftCardFromGANResponse type
#[derive(Clone, Debug, Deserialize, Default, Eq, PartialEq)]
pub struct RetrieveGiftCardFromGANResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// A gift card that was fetched, if present. It returns empty if an error occurred.
    pub gift_card: Option<GiftCard>,
}
