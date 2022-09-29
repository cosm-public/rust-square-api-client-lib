//! Model struct for CustomerPreferences type

use serde::{Deserialize, Serialize};

/// Represents communication preferences for the customer profile.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustomerPreferences {
    /// Indicates whether the customer has unsubscribed from marketing campaign emails. A value of
    /// `true` means that the customer chose to opt out of email marketing from the current Square
    /// seller or from all Square sellers. This value is read-only from the Customers API.
    pub email_unsubscribed: bool,
}
