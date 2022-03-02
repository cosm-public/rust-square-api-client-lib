//! Enum for specifying sort order in List queries

/// Enum for specifying sort order in List queries
#[derive(Debug, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Ascending
    }
}

impl From<SortOrder> for String {
    fn from(sort_order: SortOrder) -> Self {
        sort_order.to_string()
    }
}

impl ToString for SortOrder {
    fn to_string(&self) -> String {
        match self {
            SortOrder::Ascending => String::from("ASC"),
            SortOrder::Descending => String::from("DESC"),
        }
    }
}
