//! Model for CatalogDiscountModifyTaxBasis enum.

use serde::{Deserialize, Serialize};

/// Indicates whether this discount should reduce the price used to calculate tax.
///
/// Most discounts should use `MODIFY_TAX_BASIS`. However, in some circumstances taxes must be
/// calculated based on an item's price, ignoring a particular discount. For example, in many US
/// jurisdictions, a manufacturer coupon or instant rebate reduces the price a customer pays but
/// does not reduce the sale price used to calculate how much sales tax is due. In this case,
/// the discount representing that manufacturer coupon should have `DO_NOT_MODIFY_TAX_BASIS` for
/// this field.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogDiscountModifyTaxBasis {
    /// Application of the discount will modify the tax basis.
    ModifyTaxBasis,
    /// Application of the discount will not modify the tax basis.
    DoNotModifyTaxBasis,
}
