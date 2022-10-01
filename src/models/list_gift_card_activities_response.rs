//! Response struct for the List Gift Card Activities API

use serde::Deserialize;

use super::{errors::Error, GiftCardActivity};

/// This is a model struct for ListGiftCardActivitiesResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListGiftCardActivitiesResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The requested gift card activities or an empty object if none are found.
    pub gift_card_activities: Option<Vec<GiftCardActivity>>,
    /// When a response is truncated, it includes a cursor that you can use in a subsequent request
    /// to retrieve the next set of activities. If a cursor is not present, this is the final
    /// response. For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
}
