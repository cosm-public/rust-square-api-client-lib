//! Model for CardSquareProduct enum

use serde::{Deserialize, Serialize};

/// Model for CardSquareProduct enum
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardSquareProduct {
    UnknownSquareProduct,
    ConnectApi,
    Dashboard,
    RegisterClient,
    BuyerDashboard,
    Web,
    Invoices,
    GiftCard,
    VirtualTerminal,
    ReaderSdk,
    SquareProfile,
}
