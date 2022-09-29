//! Model struct for CatalogStockConversion type.

use serde::{Deserialize, Serialize};

/// Represents the rule of conversion between a stockable [CatalogItemVariation] and a non-stockable
/// sell-by or receive-by `CatalogItemVariation` that share the same underlying stock.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogStockConversion {
    /// References to the stockable [CatalogItemVariation] for this stock conversion. Selling,
    /// receiving or recounting the non-stockable `CatalogItemVariation` defined with a stock
    /// conversion results in adjustments of this stockable `CatalogItemVariation`. This immutable
    /// field must reference a stockable `CatalogItemVariation` that shares the parent [CatalogItem]
    /// of the converted `CatalogItemVariation`.
    ///
    /// Min Length 1
    pub stockable_item_variation_id: String,
    /// The quantity of the stockable item variation (as identified by
    /// `stockable_item_variation_id`) equivalent to the non-stockable item variation quantity (as
    /// specified in `nonstockable_quantity`) as defined by this stock conversion. It accepts a
    /// decimal number in a string format that can take up to 10 digits before the decimal point and
    /// up to 5 digits after the decimal point.
    ///
    /// Min Length 1 Max Length 16
    pub stockable_quantity: String,
    /// The converted equivalent quantity of the non-stockable [CatalogItemVariation] in its
    /// measurement unit. The `stockable_quantity` value and this `nonstockable_quantity` value
    /// together define the conversion ratio between stockable item variation and the non-stockable
    /// item variation. It accepts a decimal number in a string format that can take up to 10 digits
    /// before the decimal point and up to 5 digits after the decimal point.
    ///
    /// Min Length 1 Max Length 16
    pub nonstockable_quantity: String,
}
