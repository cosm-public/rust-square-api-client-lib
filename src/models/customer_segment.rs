//! Model struct for CustomerSegment type

use serde::{Deserialize, Serialize};

use super::DateTime;

/// Represents a group of customer profiles that match one or more predefined filter criteria.
///
/// Segments (also known as Smart Groups) are defined and created within the Customer Directory in
/// the Square Seller Dashboard or Point of Sale.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustomerSegment {
    /// **Read only** A unique Square-generated ID for the segment.
    pub id: Option<String>,
    /// **Read only** The name of the segment.
    pub name: String,
    /// **Read only** The timestamp when the segment was created.
    pub created_at: Option<DateTime>,
    /// **Read only** The timestamp when the segment was last updated.
    pub updated_at: Option<DateTime>,
}
