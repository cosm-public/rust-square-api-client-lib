//! Model for TransactionProduct enum

use serde::{Deserialize, Serialize};

/// Indicates the Square product used to process a transaction.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionProduct {
    /// Square Point of Sale.
    Register,
    /// The Square Connect API.
    ExternalApi,
    /// A Square subscription for one of multiple products.
    Billing,
    /// Square Appointments.
    Appointments,
    /// Square Invoices.
    Invoices,
    /// Square Online Store.
    OnlineStore,
    /// Square Payroll.
    Payroll,
    /// A Square product that does not match any other value.
    Other,
}
