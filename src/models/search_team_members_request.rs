//! Request body struct for the Search Team Members API

use serde::Serialize;

use super::SearchTeamMembersQuery;

/// This is a model struct for SearchTeamMembersRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchTeamMembersRequest {
    /// The query parameters.
    pub query: Option<SearchTeamMembersQuery>,
    /// The maximum number of `TeamMember` ojbects in a page (100 by default).
    ///
    /// Min: 1
    /// Max: 200
    pub limit: Option<i32>,
    /// The opaque cursor for fetching the next page. For more information, see
    /// [pagination](https://developer.squareup.com/docs/working-with-apis/pagination).
    pub cursor: Option<String>,
}
