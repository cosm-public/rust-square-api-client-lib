//! Model struct for CustomerGroup type

use serde::{Deserialize, Serialize};

use super::DateTime;

/// Represents a group of customer profiles.
///
/// Customer groups can be created, be modified, and have their membership defined using the
/// Customers API or within the Customer Directory in the Square Seller Dashboard or Point of Sale.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustomerGroup {
    /// **Read only** A unique Square-generated ID for the customer group.
    pub id: Option<String>,
    /// The name of the customer group.
    pub name: String,
    /// **Read only** The timestamp when the customer group was created.
    pub created_at: Option<DateTime>,
    /// **Read only** The timestamp when the customer group was last updated.
    pub updated_at: Option<DateTime>,
}
