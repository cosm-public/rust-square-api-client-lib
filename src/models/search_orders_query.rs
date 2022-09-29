//! Model struct for SearchOrdersQuery type

use serde::Serialize;

use super::{SearchOrdersFilter, SearchOrdersSort};

/// Contains query criteria for the search.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchOrdersQuery {
    /// Criteria to filter results by.
    pub filter: Option<SearchOrdersFilter>,
    /// Criteria to sort results by.
    pub sort: Option<SearchOrdersSort>,
}
