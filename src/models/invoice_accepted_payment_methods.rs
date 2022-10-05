//! Model struct for InvoiceAcceptedPaymentMethods type.

use serde::{Deserialize, Serialize};

/// The payment methods that customers can use to pay an invoice on the Square-hosted invoice page.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvoiceAcceptedPaymentMethods {
    /// Indicates whether bank transfer payments are accepted. The default value is `false`.
    ///
    /// This option is allowed only for invoices that have a single payment request of type
    /// `BALANCE`.
    pub bank_account: Option<bool>,
    /// Indicates whether credit card or debit card payments are accepted. The default value is
    /// `false`.
    pub card: Option<bool>,
    /// Indicates whether Square gift card payments are accepted. The default value is `false`.
    pub square_gift_card: Option<bool>,
}
