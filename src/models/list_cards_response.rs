//! Model struct for ListCardsResponse type

use serde::Deserialize;

use super::{errors::Error, Card};

/// This is a model struct for ListCardsResponse type
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq)]
pub struct ListCardsResponse {
    /// Information on errors encountered during the request.
    pub errors: Vec<Error>,
    /// The requested list of `Card`s.
    pub cards: Vec<Card>,
    /// The pagination cursor to be used in a subsequent request. If empty, this is the final
    /// response. See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for
    /// more information.
    pub cursor: String,
}
