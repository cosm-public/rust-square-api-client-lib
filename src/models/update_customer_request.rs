//! Model struct for UpdateCustomerRequest type

use super::{Address, CustomerTaxIds};
use serde::Serialize;
/// This is a model struct for UpdateCustomerRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateCustomerRequest {
    /// The email address associated with the customer profile.
    pub email_address: Option<String>,
    /// The family name (that is, the last name) associated with the customer profile.
    pub family_name: Option<String>,
    // The given name (that is, the first name) associated with the customer profile.
    pub given_name: Option<String>,
    /// The 11-digit phone number associated with the customer profile.
    pub phone_number: Option<String>,
    /// Represents a postal address in a country. For more information, see [Working with
    /// Addresses](https://developer.squareup.com/docs/build-basics/working-with-addresses).
    pub address: Option<Address>,
    /// The birthday associated with the customer profile, in RFC 3339 format. The year is optional.
    /// The timezone and time are not allowed. For example, `0000-09-21T00:00:00-00:00` represents a
    /// birthday on September 21 and `1998-09-21T00:00:00-00:00` represents a birthday on September
    /// 21, 1998. You can also specify this value in `YYYY-MM-DD` format.
    pub birthday: Option<String>,
    /// An optional second ID used to associate the customer profile with an entity in another system.
    pub reference_id: Option<String>,
    /// Represents the tax ID associated with a [customer profile]($m/Customer). The corresponding
    /// `tax_ids` field is available only for customers of sellers in EU countries or the United
    /// Kingdom. For more information, see [Customer tax
    /// IDs](https://developer.squareup.com/docs/customers-api/what-it-does#customer-tax-ids).
    pub tax_ids: Option<CustomerTaxIds>,
}
