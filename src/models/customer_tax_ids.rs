//! Model struct for CustomerTaxIds type

use serde::{Deserialize, Serialize};

/// Represents the tax ID associated with a [Customer] profile.
///
/// The corresponding `tax_ids` field is available only for customers of sellers in EU countries or
/// the United Kingdom. For more information, see [Customer tax
/// IDs](https://developer.squareup.com/docs/customers-api/what-it-does#customer-tax-ids).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustomerTaxIds {
    /// The EU VAT identification number for the customer. For example, `IE3426675K`. The ID can
    /// contain alphanumeric characters only.
    pub eu_vat: Option<String>,
}
