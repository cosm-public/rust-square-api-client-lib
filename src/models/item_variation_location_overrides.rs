//! Model struct for ItemVariationLocationOverrides type.

use serde::{Deserialize, Serialize};

use super::{
    enums::{CatalogPricingType, InventoryAlertType},
    DateTime, Money,
};

/// Price and inventory alerting overrides for a `CatalogItemVariation` at a specific `Location`.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ItemVariationLocationOverrides {
    /// The ID of the `Location`. This can include locations that are deactivated.
    pub location_id: Option<String>,
    /// The price of the `CatalogItemVariation` at the given `Location`, or blank for variable
    /// pricing.
    pub price_money: Option<Money>,
    /// The pricing type (fixed or variable) for the `CatalogItemVariation` at the given `Location`.
    pub pricing_type: Option<CatalogPricingType>,
    /// If `true`, inventory tracking is active for the `CatalogItemVariation` at this `Location`.
    pub track_inventory: Option<bool>,
    /// Indicates whether the `CatalogItemVariation` displays an alert when its inventory quantity
    /// is less than or equal to its `inventory_alert_threshold`.
    pub inventory_alert_type: Option<InventoryAlertType>,
    /// If the inventory quantity for the variation is less than or equal to this value and
    /// `inventory_alert_type` is `LOW_QUANTITY`, the variation displays an alert in the merchant
    /// dashboard.
    ///
    /// This value is always an integer.
    pub inventory_alert_threshold: Option<i64>,
    /// **Read only** Indicates whether the overridden item variation is sold out at the specified
    /// location.
    ///
    /// When inventory tracking is enabled on the item variation either globally or at the specified
    /// location, the item variation is automatically marked as sold out when its inventory count
    /// reaches zero. The seller can manually set the item variation as sold out even when the
    /// inventory count is greater than zero. Attempts by an application to set this attribute are
    /// ignored. Regardless how the sold-out status is set, applications should treat its inventory
    /// count as zero when this attribute value is `true`.
    pub sold_out: Option<bool>,
    /// **Read only** The seller-assigned timestamp to indicate when this sold-out variation becomes
    /// available again at the specified location. Attempts by an application to set this attribute
    /// are ignored. When the current time is later than this attribute value, the affected item
    /// variation is no longer sold out.
    pub sold_out_valid_until: Option<DateTime>,
}
