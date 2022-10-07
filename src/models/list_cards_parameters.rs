//! Model struct for ListCardsParameters (query parameters)

use super::enums::SortOrder;

const DEFAULT_SORT_ORDER: SortOrder = SortOrder::Asc;

/// This is a model struct for ListCardsParameters (query parameters)
#[derive(Clone, Debug)]
pub struct ListCardsParameters {
    /// A pagination cursor returned by a previous call to this endpoint. Provide this to
    /// retrieve the next set of results for your original query.
    /// See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for
    /// more information.
    pub cursor: String,
    /// Limit results to cards associated with the customer supplied. By default, all cards owned
    /// by the merchant are returned.
    pub customer_id: String,
    /// Includes disabled cards. By default, all enabled cards owned by the merchant are returned.
    pub include_disabled: bool,
    /// Limit results to cards associated with the reference_id supplied.
    pub reference_id: String,
    /// Sorts the returned list by when the card was created with the specified order. This field
    /// defaults to ASC.
    pub sort_order: SortOrder,
}

impl ListCardsParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl Default for ListCardsParameters {
    fn default() -> Self {
        Self {
            cursor: Default::default(),
            customer_id: Default::default(),
            include_disabled: Default::default(),
            reference_id: Default::default(),
            sort_order: DEFAULT_SORT_ORDER,
        }
    }
}

impl From<ListCardsParameters> for String {
    fn from(list_cards_parameters: ListCardsParameters) -> Self {
        list_cards_parameters.to_string()
    }
}

impl ToString for ListCardsParameters {
    fn to_string(&self) -> String {
        let mut params = Vec::new();

        if !self.cursor.is_empty() {
            params.push(format!("cursor={}", self.cursor));
        }

        if !self.customer_id.is_empty() {
            params.push(format!("customer_id={}", self.customer_id));
        }

        if self.include_disabled {
            params.push(String::from("include_disabled=true"));
        }

        if !self.reference_id.is_empty() {
            params.push(format!("reference_id={}", self.reference_id));
        }

        if self.sort_order != DEFAULT_SORT_ORDER {
            params.push(format!("sort_order={}", serde_json::to_string(&self.sort_order).unwrap()));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}
