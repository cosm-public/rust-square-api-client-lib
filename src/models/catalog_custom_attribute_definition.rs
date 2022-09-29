//! Model struct for CatalogCustomAttributeDefinition type.

use serde::{Deserialize, Serialize};

use super::{
    enums::{
        CatalogCustomAttributeDefinitionAppVisibility,
        CatalogCustomAttributeDefinitionSellerVisibility, CatalogCustomAttributeDefinitionType,
        CatalogObjectType,
    },
    CatalogCustomAttributeDefinitionNumberConfig, CatalogCustomAttributeDefinitionSelectionConfig,
    CatalogCustomAttributeDefinitionStringConfig, SourceApplication,
};

/// Contains information defining a custom attribute.
///
/// Custom attributes are intended to store additional information about a catalog object or to
/// associate a catalog object with an entity in another system. Do not use custom attributes to
/// store any sensitive information (personally identifiable information, card details, etc.). [Read
/// more about custom
/// attributes](https://developer.squareup.com/docs/catalog-api/add-custom-attributes)
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogCustomAttributeDefinition {
    /// The type of this custom attribute. Cannot be modified after creation. Required.
    pub r#type: CatalogCustomAttributeDefinitionType,
    /// The name of this definition for API and seller-facing UI purposes. The name must be unique
    /// within the (merchant, application) pair. Required. May not be empty and may not exceed 255
    /// characters. Can be modified after creation.
    pub name: String,
    /// Seller-oriented description of the meaning of this Custom Attribute, any constraints that
    /// the seller should observe, etc. May be displayed as a tooltip in Square UIs.
    pub description: Option<String>,
    /// **Read only.** Contains information about the application that created this custom attribute
    /// definition.
    pub source_application: Option<SourceApplication>,
    /// The set of Catalog Object Types that this Custom Attribute may be applied to. Currently,
    /// only `ITEM` and `ITEM_VARIATION` are allowed. At least one type must be included.
    pub allowed_object_types: Vec<CatalogObjectType>,
    /// The visibility of a custom attribute in seller-facing UIs (including Square Point of Sale
    /// applications and Square Dashboard). May be modified.
    pub seller_visibility: Option<CatalogCustomAttributeDefinitionSellerVisibility>,
    /// The visibility of a custom attribute to applications other than the application that created
    /// the attribute.
    pub app_visibility: Option<CatalogCustomAttributeDefinitionAppVisibility>,
    /// Optionally, populated when `type` = `STRING`, unset otherwise.
    pub string_config: Option<CatalogCustomAttributeDefinitionStringConfig>,
    /// Optionally, populated when `type` = `NUMBER`, unset otherwise.
    pub number_config: Option<CatalogCustomAttributeDefinitionNumberConfig>,
    /// Populated when `type` is set to `SELECTION`, unset otherwise.
    pub selection_config: Option<CatalogCustomAttributeDefinitionSelectionConfig>,
    /// **Read-only.** The number of custom attributes that reference this custom attribute
    /// definition. Set by the server in response to a ListCatalog request with `include_counts` set
    /// to `true`. If the actual count is greater than 100, `custom_attribute_usage_count` will be
    /// set to `100`.
    pub custom_attribute_usage_count: Option<i32>,
    /// The name of the desired custom attribute key that can be used to access the custom attribute
    /// value on catalog objects. Cannot be modified after the custom attribute definition has been
    /// created. Must be between 1 and 60 characters, and may only contain the characters
    /// `[a-zA-Z0-9_-]`.
    pub key: Option<String>,
}
