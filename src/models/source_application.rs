//! Model struct for SourceApplication type.

use serde::{Deserialize, Serialize};

use super::enums::Product;

/// Provides information about the application used to generate a change.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceApplication {
    /// **Read-only** [Product] type for the application.
    pub product: Option<Product>,
    /// **Read-only** Square ID assigned to the application. Only used for [Product] type
    /// `EXTERNAL_API`.
    pub application_id: Option<String>,
    /// **Read-only** display name assigned to the application (e.g. `"Custom Application"`,
    /// `"Square POS 4.74 for Android"`).
    pub name: Option<String>,
}
