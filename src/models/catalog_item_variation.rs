//! Model struct for CatalogItemVariation type.

use serde::{Deserialize, Serialize};

use super::{
    enums::{CatalogPricingType, InventoryAlertType},
    CatalogItemOptionValueForItemVariation, CatalogStockConversion, ItemVariationLocationOverrides,
    Money,
};

/// An item variation (i.e., product) in the Catalog object model.
///
/// Each item may have a maximum of 250 item variations.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogItemVariation {
    /// The ID of the `CatalogItem` associated with this item variation.
    pub item_id: Option<String>,
    /// The item variation's name. This is a searchable attribute for use in applicable query
    /// filters, and its value length is of Unicode code points.
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// The item variation's SKU, if any. This is a searchable attribute for use in applicable query
    /// filters.
    pub sku: Option<String>,
    /// The universal product code (UPC) of the item variation, if any. This is a searchable
    /// attribute for use in applicable query filters.
    ///
    /// The value of this attribute should be a number of 12-14 digits long. This restriction is
    /// enforced on the Square Seller Dashboard, Square Point of Sale or Retail Point of Sale apps,
    /// where this attribute shows in the GTIN field. If a non-compliant UPC value is assigned to
    /// this attribute using the API, the value is not editable on the Seller Dashboard, Square
    /// Point of Sale or Retail Point of Sale apps unless it is updated to fit the expected format.
    pub upc: Option<String>,
    /// **Read only** The order in which this item variation should be displayed. This value is
    /// read-only. On writes, the ordinal for each item variation within a parent `CatalogItem` is
    /// set according to the item variations's position. On reads, the value is not guaranteed to be
    /// sequential or unique.
    pub ordinal: Option<i32>,
    /// Indicates whether the item variation's price is fixed or determined at the time of sale.
    pub pricing_type: Option<CatalogPricingType>,
    /// The item variation's price, if fixed pricing is used.
    pub price_money: Option<Money>,
    /// Per-location price and inventory overrides.
    pub location_overrides: Option<Vec<ItemVariationLocationOverrides>>,
    /// If `true`, inventory tracking is active for the variation.
    pub track_inventory: Option<bool>,
    /// Indicates whether the item variation displays an alert when its inventory quantity is less
    /// than or equal to its `inventory_alert_threshold`.
    pub inventory_alert_type: Option<InventoryAlertType>,
    /// If the inventory quantity for the variation is less than or equal to this value and
    /// `inventory_alert_type` is `LOW_QUANTITY`, the variation displays an alert in the merchant
    /// dashboard.
    ///
    /// This value is always an integer.
    pub inventory_alert_threshold: Option<i64>,
    /// Arbitrary user metadata to associate with the item variation. This attribute value length is
    /// of Unicode code points.
    ///
    /// Max Length 255
    pub user_data: Option<String>,
    /// If the `CatalogItem` that owns this item variation is of type `APPOINTMENTS_SERVICE`, then
    /// this is the duration of the service in milliseconds. For example, a 30 minute appointment
    /// would have the value `1800000`, which is equal to 30 (minutes) * 60 (seconds per minute) *
    /// 1000 (milliseconds per second).
    pub service_duration: Option<i64>,
    /// If the `CatalogItem` that owns this item variation is of type `APPOINTMENTS_SERVICE`, a bool
    /// representing whether this service is available for booking.
    pub available_for_booking: Option<bool>,
    /// List of item option values associated with this item variation. Listed in the same order as
    /// the item options of the parent item.
    pub item_option_values: Option<Vec<CatalogItemOptionValueForItemVariation>>,
    /// ID of the `CatalogMeasurementUnit` that is used to measure the quantity sold of this item
    /// variation. If left unset, the item will be sold in whole quantities.
    pub measurement_unit_id: Option<String>,
    /// Whether this variation can be sold.
    pub sellable: Option<bool>,
    /// Whether stock is counted directly on this variation (TRUE) or only on its components
    /// (FALSE).
    pub stockable: Option<bool>,
    /// The IDs of images associated with this `CatalogItemVariation` instance. These images will be
    /// shown to customers in Square Online Store.
    pub image_ids: Option<Vec<String>>,
    /// Tokens of employees that can perform the service represented by this variation. Only valid
    /// for variations of type `APPOINTMENTS_SERVICE`.
    pub team_member_ids: Option<Vec<String>>,
    /// The rule of conversion of the [CatalogStockConversion] type that describes how this
    /// non-stockable sellable/receivable item variation is converted to/from the stockable item
    /// variation sharing the same parent item.
    pub stockable_conversion: Option<CatalogStockConversion>,
}
