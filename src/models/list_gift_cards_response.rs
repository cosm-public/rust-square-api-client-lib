//! Response struct for the List Gift Cards API

use serde::Deserialize;

use super::{errors::Error, GiftCard};

/// This is a model struct for ListGiftCardsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListGiftCardsResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The requested gift cards or an empty object if none are found.
    pub gift_cards: Option<Vec<GiftCard>>,
    /// When a response is truncated, it includes a cursor that you can use in a subsequent request
    /// to retrieve the next set of gift cards. If a cursor is not present, this is the final
    /// response. For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
}
