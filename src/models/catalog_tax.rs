//! Model struct for CatalogTax type.

use serde::{Deserialize, Serialize};

use super::enums::{TaxCalculationPhase, TaxInclusionType};

/// A tax applicable to an item.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogTax {
    /// The tax's name. This is a searchable attribute for use in applicable query filters, and its
    /// value length is of Unicode code points.
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// Whether the tax is calculated based on a payment's subtotal or total.
    pub calculation_phase: Option<TaxCalculationPhase>,
    /// Whether the tax is `ADDITIVE` or `INCLUSIVE`.
    pub inclusion_type: Option<TaxInclusionType>,
    /// The percentage of the tax in decimal form, using a `'.'` as the decimal separator and
    /// without a `'%'` sign. A value of `7.5` corresponds to 7.5%.
    pub percentage: Option<String>,
    /// If `true`, the fee applies to custom amounts entered into the Square Point of Sale app that
    /// are not associated with a particular `CatalogItem`.
    pub applies_to_custom_amounts: Option<bool>,
    /// A Boolean flag to indicate whether the tax is displayed as enabled (`true`) in the Square
    /// Point of Sale app or not (`false`).
    pub enabled: Option<bool>,
}
