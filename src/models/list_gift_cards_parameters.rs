//! Query parameters for the List Gift Cards API

use super::enums::{GiftCardStatus, GiftCardType};

/// This is a model struct for ListGiftCardsParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListGiftCardsParameters {
    /// If a type is provided, the endpoint returns gift cards of the specified type. Otherwise, the
    /// endpoint returns gift cards of all types.
    pub r#type: Option<GiftCardType>,
    /// If a state is provided, the endpoint returns the gift cards in the specified state.
    /// Otherwise, the endpoint returns the gift cards of all states.
    pub state: Option<GiftCardStatus>,
    /// If a limit is provided, the endpoint returns only the specified number of results per page.
    /// The maximum value is 50. The default value is 30. For more information, see
    /// [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination).
    pub limit: Option<i32>,
    /// A pagination cursor returned by a previous call to this endpoint. Provide this cursor to
    /// retrieve the next set of results for the original query. If a cursor is not provided, the
    /// endpoint returns the first page of the results. For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// If a customer ID is provided, the endpoint returns only the gift cards linked to the
    /// specified customer.
    pub customer_id: Option<String>,
}

impl ListGiftCardsParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListGiftCardsParameters> for String {
    fn from(list_gift_cards_parameters: ListGiftCardsParameters) -> Self {
        list_gift_cards_parameters.to_string()
    }
}

impl ToString for ListGiftCardsParameters {
    fn to_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(gift_card_type) = &self.r#type {
            params.push(format!("type={}", serde_json::to_string(gift_card_type).unwrap()));
        }

        if let Some(gift_card_status) = &self.state {
            params.push(format!("state={}", serde_json::to_string(gift_card_status).unwrap()));
        }

        if let Some(limit) = self.limit {
            params.push(format!("limit={}", limit));
        }

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(customer_id) = &self.customer_id {
            params.push(format!("customer_id={}", customer_id));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}
