//! Model for LocationType enum

use serde::{Deserialize, Serialize};

/// A location's type.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LocationType {
    /// A place of business with a physical location.
    Physical,
    /// A place of business that is mobile, such as a food truck or online store.
    Mobile,
}
