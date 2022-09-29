//! Model struct for SearchCustomersResponse type

use serde::Deserialize;

use super::{errors::Error, Customer};

/// This is a model struct for SearchCustomersResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct SearchCustomersResponse {
    /// A list of [Customer] objects that match the query conditions. The list is populated only if
    /// `return_entries` is set to `false` in the request.
    pub customers: Option<Vec<Customer>>,
    /// The pagination cursor to be used in a subsequent request. If unset, this is the final
    /// response. For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// [Error]s encountered during the search
    pub errors: Option<Vec<Error>>,
}
