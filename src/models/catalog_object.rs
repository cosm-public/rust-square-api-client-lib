//! Model struct for CatalogObject type

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    enums::CatalogObjectType, CatalogCategory, CatalogCustomAttributeDefinition,
    CatalogCustomAttributeValue, CatalogDiscount, CatalogImage, CatalogItem, CatalogItemOption,
    CatalogItemOptionValue, CatalogItemVariation, CatalogMeasurementUnit, CatalogModifier,
    CatalogModifierList, CatalogPricingRule, CatalogProductSet, CatalogQuickAmountsSettings,
    CatalogSubscriptionPlan, CatalogTax, CatalogTimePeriod, CatalogV1Id, DateTime,
};

/// The wrapper object for the catalog entries of a given object type.
///
/// Depending on the `type` attribute value, a `CatalogObject` instance assumes a type-specific data
/// to yield the corresponding type of catalog object.
///
/// For example, if `type=ITEM`, the `CatalogObject` instance must have the ITEM-specific data set
/// on the `item_data` attribute. The resulting `CatalogObject` instance is also a `CatalogItem`
/// instance.
///
/// In general, if `type=<OBJECT_TYPE>`, the `CatalogObject` instance must have the
/// `<OBJECT_TYPE>`-specific data set on the `<object_type>_data` attribute. The resulting
/// `CatalogObject` instance is also a `Catalog<ObjectType>` instance.
///
/// For a more detailed discussion of the Catalog data model, please see the
/// [Design a Catalog](https://developer.squareup.com/docs/catalog-api/design-a-catalog) guide.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogObject {
    /// The type of this object. Each object type has expected properties expressed in a structured
    /// format within its corresponding `*_data` field below.
    pub r#type: CatalogObjectType,
    /// An identifier to reference this object in the catalog. When a new `CatalogObject` is
    /// inserted, the client should set the id to a temporary identifier starting with a "`#`"
    /// character. Other objects being inserted or updated within the same request may use this
    /// identifier to refer to the new object.
    ///
    /// When the server receives the new object, it will supply a unique identifier that replaces
    /// the temporary identifier for all future references.
    pub id: String,
    /// **Read only** Last modification timestamp.
    pub updated_at: Option<DateTime>,
    /// The version of the object. When updating an object, the version supplied must match the
    /// version in the database, otherwise the write will be rejected as conflicting.
    pub version: Option<i64>,
    /// If `true`, the object has been deleted from the database. Must be `false` for new objects
    /// being inserted. When deleted, the `updated_at` field will equal the deletion time.
    pub is_deleted: Option<bool>,
    /// A map (key-value pairs) of application-defined custom attribute values. The value of a
    /// key-value pair is a [CatalogCustomAttributeValue] object. The key is the `key` attribute
    /// value defined in the associated [CatalogCustomAttributeDefinition] object defined by the
    /// application making the request.
    ///
    /// If the `CatalogCustomAttributeDefinition` object is defined by another application, the
    /// `CatalogCustomAttributeDefinition`'s key attribute value is prefixed by the defining
    /// application ID. For example, if the `CatalogCustomAttributeDefinition` has a `key` attribute
    /// of `"cocoa_brand"` and the defining application ID is `"abcd1234"`, the key in the map is
    /// `"abcd1234:cocoa_brand"` if the application making the request is different from the
    /// application defining the custom attribute definition. Otherwise, the key used in the map is
    /// simply `"cocoa_brand"`.
    ///
    /// Application-defined custom attributes are set at a global (location-independent) level.
    /// Custom attribute values are intended to store additional information about a catalog object
    /// or associations with an entity in another system. Do not use custom attributes to store any
    /// sensitive information (personally identifiable information, card details, etc.).
    pub custom_attribute_values: Option<HashMap<String, CatalogCustomAttributeValue>>,
    /// The Connect v1 IDs for this object at each location where it is present, where they differ
    /// from the object's Connect V2 ID. The field will only be present for objects that have been
    /// created or modified by legacy APIs.
    pub catalog_v1_ids: Option<Vec<CatalogV1Id>>,
    /// If `true`, this object is present at all locations (including future locations), except
    /// where specified in the `absent_at_location_ids` field. If `false`, this object is not
    /// present at any locations (including future locations), except where specified in the
    /// `present_at_location_ids` field. If not specified, defaults to `true`.
    pub present_at_all_locations: Option<bool>,
    /// A list of locations where the object is present, even if `present_at_all_locations` is
    /// `false`. This can include locations that are deactivated.
    pub present_at_location_ids: Option<Vec<String>>,
    /// A list of locations where the object is not present, even if `present_at_all_locations` is
    /// `true`. This can include locations that are deactivated.
    pub absent_at_location_ids: Option<Vec<String>>,
    /// Structured data for a `CatalogItem`, set for CatalogObjects of type `ITEM`.
    pub item_data: Option<CatalogItem>,
    /// Structured data for a `CatalogCategory`, set for CatalogObjects of type `CATEGORY`.
    pub category_data: Option<CatalogCategory>,
    /// Structured data for a `CatalogItemVariation`, set for CatalogObjects of type
    /// `ITEM_VARIATION`.
    pub item_variation_data: Option<CatalogItemVariation>,
    /// Structured data for a `CatalogTax`, set for CatalogObjects of type `TAX`.
    pub tax_data: Option<CatalogTax>,
    /// Structured data for a `CatalogDiscount`, set for CatalogObjects of type `DISCOUNT`.
    pub discount_data: Option<CatalogDiscount>,
    /// Structured data for a `CatalogModifierList`, set for CatalogObjects of type `MODIFIER_LIST`.
    pub modifier_list_data: Option<CatalogModifierList>,
    /// Structured data for a `CatalogModifier`, set for CatalogObjects of type `MODIFIER`.
    pub modifier_data: Option<CatalogModifier>,
    /// Structured data for a `CatalogTimePeriod`, set for CatalogObjects of type `TIME_PERIOD`.
    pub time_period_data: Option<CatalogTimePeriod>,
    /// Structured data for a `CatalogProductSet`, set for CatalogObjects of type `PRODUCT_SET`.
    pub product_set_data: Option<CatalogProductSet>,
    /// Structured data for a `CatalogPricingRule`, set for CatalogObjects of type `PRICING_RULE`. A
    /// `CatalogPricingRule` object often works with a `CatalogProductSet` object or a
    /// `CatalogTimePeriod` object.
    pub pricing_rule_data: Option<CatalogPricingRule>,
    /// Structured data for a `CatalogImage`, set for CatalogObjects of type `IMAGE`.
    pub image_data: Option<CatalogImage>,
    /// Structured data for a `CatalogMeasurementUnit`, set for CatalogObjects of type
    /// `MEASUREMENT_UNIT`.
    pub measurement_unit_data: Option<CatalogMeasurementUnit>,
    /// Structured data for a `CatalogSubscriptionPlan`, set for CatalogObjects of type
    /// `SUBSCRIPTION_PLAN`.
    pub subscription_plan_data: Option<CatalogSubscriptionPlan>,
    /// Structured data for a `CatalogItemOption`, set for CatalogObjects of type `ITEM_OPTION`.
    pub item_option_data: Option<CatalogItemOption>,
    /// Structured data for a `CatalogItemOptionValue`, set for CatalogObjects of type
    /// `ITEM_OPTION_VAL`.
    pub item_option_value_data: Option<CatalogItemOptionValue>,
    /// Structured data for a `CatalogCustomAttributeDefinition`, set for CatalogObjects of type
    /// `CUSTOM_ATTRIBUTE_DEFINITION`.
    pub custom_attribute_definition_data: Option<CatalogCustomAttributeDefinition>,
    /// Structured data for a `CatalogQuickAmountsSettings`, set for CatalogObjects of type
    /// `QUICK_AMOUNTS_SETTINGS`.
    pub quick_amounts_settings_data: Option<CatalogQuickAmountsSettings>,
}
