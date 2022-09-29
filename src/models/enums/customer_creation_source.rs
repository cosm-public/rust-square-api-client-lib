//! Model for CustomerCreationSource enum

use serde::{Deserialize, Serialize};

/// Indicates the method used to create the customer profile.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomerCreationSource {
    /// The default creation source. This source is typically used for backward/future compatibility
    /// when the original source of a customer profile is unrecognized. For example, when older
    /// clients do not support newer source types.
    Other,
    /// The customer profile was created automatically when an appointment was scheduled.
    Appointments,
    /// The customer profile was created automatically when a coupon was issued using Square Point
    /// of Sale.
    Coupon,
    /// The customer profile was restored through Square's deletion recovery process.
    DeletionRecovery,
    /// The customer profile was created manually through Square Seller Dashboard or the Point of
    /// Sale application.
    Directory,
    /// The customer profile was created automatically when a gift card was issued using Square
    /// Point of Sale. Customer profiles are created for both the buyer and the recipient of the
    /// gift card.
    Egifting,
    /// The customer profile was created through Square Point of Sale when signing up for marketing
    /// emails during checkout.
    EmailCollection,
    /// The customer profile was created automatically when providing feedback through a digital
    /// receipt.
    Feedback,
    /// The customer profile was created automatically when importing customer data through Square
    /// Seller Dashboard.
    Import,
    /// The customer profile was created automatically during an invoice payment.
    Invoices,
    /// The customer profile was created automatically when customers provide a phone number for
    /// loyalty reward programs during checkout.
    Loyalty,
    /// The customer profile was created as the result of a campaign managed through Square’s
    /// Facebook integration.
    Marketing,
    /// The customer profile was created as the result of explicitly merging multiple customer
    /// profiles through the Square Seller Dashboard or the Point of Sale application.
    Merge,
    /// The customer profile was created through Square's Online Store solution (legacy service).
    OnlineStore,
    /// The customer profile was created automatically as the result of a successful transaction
    /// that did not explicitly link to an existing customer profile.
    InstantProfile,
    /// The customer profile was created through Square's Virtual Terminal.
    Terminal,
    /// The customer profile was created through a Square API call.
    ThirdParty,
    /// The customer profile was created by a third-party product and imported through an official
    /// integration.
    ThirdPartyImport,
    /// The customer profile was restored through Square's unmerge recovery process.
    UnmergeRecovery,
}
