//! Model struct for Customer type

use serde::{Deserialize, Serialize};

use super::{
    enums::CustomerCreationSource, Address, Card, CustomerPreferences, CustomerTaxIds, DateTime,
};

/// Represents a Square customer profile in the Customer Directory of a Square seller.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Customer {
    /// A unique Square-assigned ID for the customer profile. If you need this ID for an API
    /// request, use the ID returned when you created the customer profile or call the
    /// `SearchCustomers` or `ListCustomers` endpoint.
    pub id: Option<String>,
    /// The timestamp when the customer profile was created.
    pub created_at: Option<DateTime>,
    /// The timestamp when the customer profile was last updated.
    pub updated_at: Option<DateTime>,
    /// Payment details of the credit, debit, and gift cards stored on file for the customer
    /// profile.
    ///
    /// DEPRECATED at version 2021-06-16. Replaced by calling `ListCards` (for credit and debit
    /// cards on file) or `ListGiftCards` (for gift cards on file) and including the
    /// `customer_id` query parameter. For more information, see [Migrate to the Cards API and Gift
    /// Cards
    /// API](https://developer.squareup.com/docs/customers-api/use-the-api/integrate-with-other-services#migrate-customer-cards).
    #[deprecated]
    pub cards: Option<Vec<Card>>,
    /// The given name (that is, the first name) associated with the customer profile.
    pub given_name: Option<String>,
    /// The family name (that is, the last name) associated with the customer profile.
    pub family_name: Option<String>,
    /// A nickname for the customer profile.
    pub nickname: Option<String>,
    /// A business name associated with the customer profile.
    pub company_name: Option<String>,
    /// The email address associated with the customer profile.
    pub email_address: Option<String>,
    /// The physical address associated with the customer profile.
    pub address: Option<Address>,
    /// The phone number associated with the customer profile. A phone number can contain 9–16
    /// digits, with an optional `+` prefix.
    pub phone_number: Option<String>,
    /// The birthday associated with the customer profile, in RFC 3339 format. The year is optional.
    /// The timezone and time are not allowed. For example, `0000-09-21T00:00:00-00:00` represents a
    /// birthday on September 21 and `1998-09-21T00:00:00-00:00` represents a birthday on September
    /// 21, 1998.
    pub birthday: Option<String>,
    /// An optional second ID used to associate the customer profile with an entity in another
    /// system.
    pub reference_id: Option<String>,
    /// A custom note associated with the customer profile.
    pub note: Option<String>,
    /// Represents general customer preferences.
    pub preferences: Option<CustomerPreferences>,
    /// The method used to create the customer profile.
    pub creation_source: Option<CustomerCreationSource>,
    /// The IDs of customer groups the customer belongs to.
    pub group_ids: Option<Vec<String>>,
    /// The IDs of segments the customer belongs to.
    pub segment_ids: Option<Vec<String>>,
    /// The Square-assigned version number of the customer profile. The version number is
    /// incremented each time an update is committed to the customer profile, except for changes to
    /// customer segment membership and cards on file.
    pub version: Option<i32>,
    /// Represents the tax ID associated with a [customer profile]($m/Customer). The corresponding
    /// `tax_ids` field is available only for customers of sellers in EU countries or the United
    /// Kingdom. For more information, see [Customer tax
    /// IDs](https://developer.squareup.com/docs/customers-api/what-it-does#customer-tax-ids).
    pub tax_ids: Option<CustomerTaxIds>,
}
