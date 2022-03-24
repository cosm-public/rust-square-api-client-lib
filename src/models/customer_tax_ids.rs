//! Model struct for CustomerTaxIds type

use serde::{Deserialize, Serialize};

/// This is a model struct for CustomerTaxIds type
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CustomerTaxIds {
    /// The EU VAT identification number for the customer. For example, `IE3426675K`. The ID can
    /// contain alphanumeric characters only.
    pub eu_vat: Option<String>,
}
