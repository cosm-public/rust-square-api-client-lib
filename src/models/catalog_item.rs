//! Model struct for CatalogItem type.

use serde::{Deserialize, Serialize};

use super::{
    enums::CatalogItemProductType, CatalogItemModifierListInfo, CatalogItemOptionForItem,
    CatalogObject,
};

/// A [CatalogObject] instance of the `ITEM` type, also referred to as an item, in the catalog.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogItem {
    /// The item's name. This is a searchable attribute for use in applicable query filters, its
    /// value must not be empty, and the length is of Unicode code points.
    pub name: Option<String>,
    /// The item's description. This is a searchable attribute for use in applicable query filters,
    /// and its value length is of Unicode code points.
    pub description: Option<String>,
    /// The text of the item's display label in the Square Point of Sale app. Only up to the first
    /// five characters of the string are used. This attribute is searchable, and its value length
    /// is of Unicode code points.
    pub abbreviation: Option<String>,
    /// The color of the item's display label in the Square Point of Sale app. This must be a valid
    /// hex color code.
    pub label_color: Option<String>,
    /// If `true`, the item can be added to shipping orders from the merchant's online store.
    pub available_online: Option<bool>,
    /// If `true`, the item can be added to pickup orders from the merchant's online store.
    pub available_for_pickup: Option<bool>,
    /// If `true`, the item can be added to electronically fulfilled orders from the merchant's
    /// online store.
    pub available_electronically: Option<bool>,
    /// The ID of the item's category, if any.
    pub category_id: Option<String>,
    /// A set of IDs indicating the taxes enabled for this item. When updating an item, any taxes
    /// listed here will be added to the item. Taxes may also be added to or deleted from an item
    /// using `UpdateItemTaxes`.
    pub tax_ids: Option<Vec<String>>,
    /// A set of `CatalogItemModifierListInfo` objects representing the modifier lists that apply to
    /// this item, along with the overrides and min and max limits that are specific to this item.
    /// Modifier lists may also be added to or deleted from an item using `UpdateItemModifierLists`.
    pub modifier_list_info: Option<Vec<CatalogItemModifierListInfo>>,
    /// A list of [CatalogItemVariation] objects for this item. An item must have at least one
    /// variation.
    pub variations: Option<Vec<CatalogObject>>,
    /// The product type of the item. May not be changed once an item has been created.
    ///
    /// Only items of product type `REGULAR` or `APPOINTMENTS_SERVICE` may be created by this API;
    /// items with other product types are read-only.
    pub product_type: Option<CatalogItemProductType>,
    /// If `false`, the Square Point of Sale app will present the `CatalogItem`'s details screen
    /// immediately, allowing the merchant to choose `CatalogModifier`s before adding the item to
    /// the cart. This is the default behavior.
    ///
    /// If `true`, the Square Point of Sale app will immediately add the item to the cart with the
    /// pre-selected modifiers, and merchants can edit modifiers by drilling down onto the item's
    /// details.
    ///
    /// Third-party clients are encouraged to implement similar behaviors.
    pub skip_modifier_screen: Option<bool>,
    /// List of item options IDs for this item. Used to manage and group item variations in a
    /// specified order.
    ///
    /// Maximum: 6 item options.
    pub item_options: Option<Vec<CatalogItemOptionForItem>>,
    /// The IDs of images associated with this `CatalogItem` instance. These images will be shown to
    /// customers in Square Online Store. The first image will show up as the icon for this item in
    /// POS.
    pub image_ids: Option<Vec<String>>,
    /// A name to sort the item by. If this name is unspecified, namely, the `sort_name` field is
    /// absent, the regular `name` field is used for sorting.
    ///
    /// It is currently supported for sellers of the Japanese locale only.
    pub sort_name: Option<String>,
}
