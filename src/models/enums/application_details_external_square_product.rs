//! Model for ApplicationDetailsExternalSquareProduct enum.

use serde::{Deserialize, Serialize};

/// A list of products to return to external callers.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApplicationDetailsExternalSquareProduct {
    Appointments,
    EcommerceApi,
    Invoices,
    OnlineStore,
    Other,
    Restaurants,
    Retail,
    SquarePos,
    TerminalApi,
    VirtualTerminal,
}
