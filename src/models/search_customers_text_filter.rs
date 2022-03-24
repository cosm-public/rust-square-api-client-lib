//! Model struct for SearchCustomerTextFilter type

/// This is a model struct for SearchCustomerTextFilter type
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct SearchCustomersTextFilter {
    /// For exact matching, this filter causes the search to return customer profiles
    /// whose email_address field value are identical to the email address provided in the query.
    pub exact: Option<String>,
    /// For fuzzy matching, this filter causes the search to return customer profiles
    /// whose email_address field value has a token-wise partial match against the
    /// filtering expression in the query For example, with Steven gmail provided
    /// in a search query,the search returns customers whose email address can be
    /// steven.johnson@gmail.com or mygmail@stevensbakery.com.
    pub fuzzy: Option<String>,
}
