//! Request body struct for the Retrieve Gift Card From GAN API

use serde::Serialize;

/// This is a model struct for RetrieveGiftCardFromGANRequest type
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct RetrieveGiftCardFromGANRequest {
    /// The gift card account number (GAN) of the gift card to retrieve. The maximum length of a GAN
    /// is 255 digits to account for third-party GANs that have been imported. Square-issued gift
    /// cards have 16-digit GANs.
    ///
    /// Min Length: 1, Max Length: 255
    pub gan: String,
}
