//! Model for SortCustomersField enum

use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]

/// Use one or more customer attributes as the sort key to sort searched customer profiles.
/// For example, use the creation date (created_at) of customers or default attributes as the sort key.
pub enum SortCustomersField {
    /// By default, customers are sorted alphanumerically by concatenating their given_name and family_name.
    Default,
    /// Use the creation date attribute (created_at) of customer profiles as the sort key.
    CreatedAt,
}
impl Default for SortCustomersField {
    fn default() -> Self {
        SortCustomersField::Default
    }
}

impl From<SortCustomersField> for String {
    fn from(sort_field: SortCustomersField) -> Self {
        sort_field.to_string()
    }
}

impl ToString for SortCustomersField {
    fn to_string(&self) -> String {
        match self {
            SortCustomersField::Default => String::from("DEFAULT"),
            SortCustomersField::CreatedAt => String::from("CREATED_AT"),
        }
    }
}
