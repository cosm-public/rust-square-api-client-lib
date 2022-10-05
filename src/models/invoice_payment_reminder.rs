//! Model struct for InvoicePaymentReminder type.

use serde::{Deserialize, Serialize};

use super::{enums::InvoicePaymentReminderStatus, DateTime};

/// Describes a payment request reminder (automatic notification) that Square sends to the customer.
///
/// You configure a reminder relative to the payment request `due_date`.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvoicePaymentReminder {
    /// The reminder message.
    ///
    /// Min Length: 1, Max Length: 1000
    pub message: Option<String>,
    /// The number of days before (a negative number) or after (a positive number) the payment
    /// request `due_date` when the reminder is sent. For example, -3 indicates that the reminder
    /// should be sent 3 days before the payment request `due_date`.
    ///
    /// Min: -32767, Max: 32767
    pub relative_scheduled_days: Option<i32>,
    /// **Read only** If sent, the timestamp when the reminder was sent, in RFC 3339 format.
    pub sent_at: Option<DateTime>,
    /// **Read only** The status of the reminder.
    pub status: Option<InvoicePaymentReminderStatus>,
    /// **Read only** A Square-assigned ID that uniquely identifies the reminder within the
    /// `InvoicePaymentRequest`.
    pub uid: Option<String>,
}
