//! Model struct for OrderFulfillmentRecipient type

use serde::{Deserialize, Serialize};

use super::Address;

/// Contains information about the recipient of a fulfillment.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderFulfillmentRecipient {
    /// The customer ID of the customer associated with the fulfillment.
    ///
    /// If `customer_id` is provided, the fulfillment recipient's `display_name`, `email_address`,
    /// and `phone_number` are automatically populated from the targeted customer profile. If these
    /// fields are set in the request, the request values overrides the information from the
    /// customer profile. If the targeted customer profile does not contain the necessary
    /// information and these fields are left unset, the request results in an error.
    pub customer_id: Option<String>,
    /// The display name of the fulfillment recipient.
    ///
    /// If provided, the display name overrides the value pulled from the customer profile indicated
    /// by `customer_id`.
    pub display_name: Option<String>,
    /// The email address of the fulfillment recipient.
    ///
    /// If provided, the email address overrides the value pulled from the customer profile
    /// indicated by `customer_id`.
    pub email_address: Option<String>,
    /// The phone number of the fulfillment recipient.
    ///
    /// If provided, the phone number overrides the value pulled from the customer profile indicated
    /// by `customer_id`.
    pub phone_number: Option<String>,
    /// The address of the fulfillment recipient.
    ///
    /// If provided, the address overrides the value pulled from the customer profile indicated by
    /// `customer_id`.
    pub address: Option<Address>,
}
