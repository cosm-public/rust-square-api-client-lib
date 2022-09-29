//! Model struct for ListCustomersResponse type

use serde::{Deserialize, Serialize};

use super::{errors::Error, Customer};

/// This is a model struct for ListCustomersResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListCustomersResponse {
    /// Information on errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The requested list of `Customers`.
    pub customers: Option<Vec<Customer>>,
    /// The pagination cursor to be used in a subsequent request. If empty, this is the final
    /// response. See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for
    /// more information.
    pub cursor: Option<String>,
}
