//! Request body struct for the Create Gift Card Activity API

use serde::Serialize;

use super::GiftCardActivity;

/// This is a model struct for CreateGiftCardActivityRequest type
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CreateGiftCardActivityRequest {
    /// A unique string that identifies the CreateGiftCardActivity request.
    ///
    /// Min Length: 1 Max Length: 128
    pub idempotency_key: String,
    /// The activity to create for the gift card. This activity must specify `gift_card_id` or
    /// `gift_card_gan` for the target gift card, the `location_id` where the activity occurred, and
    /// the activity `type` along with the corresponding activity details.
    pub gift_card_activity: GiftCardActivity,
}
