//! Model struct for RetrieveInventoryCountParams type

#[derive(Clone, Debug, Default)]
pub struct RetrieveInventoryCountParams {
    pub location_ids: Option<Vec<String>>,
    /// The pagination cursor returned in the previous response. Leave unset for an initial request.
    /// The page size is currently set to be 100.
    /// See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for
    /// more information.
    pub cursor: Option<String>,
}

impl RetrieveInventoryCountParams {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<RetrieveInventoryCountParams> for String {
    fn from(retrieve_inventory_count_parameters: RetrieveInventoryCountParams) -> Self {
        retrieve_inventory_count_parameters.to_string()
    }
}

impl ToString for RetrieveInventoryCountParams {
    fn to_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(location_ids) = &self.location_ids {
            params.push(format!("location_ids={}", location_ids.join(",")));
        }

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}
