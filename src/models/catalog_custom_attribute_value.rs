//! Model struct for CatalogCustomAttributeValue type.

use serde::{Deserialize, Serialize};

use super::enums::CatalogCustomAttributeDefinitionType;

/// An instance of a custom attribute.
///
/// Custom attributes can be defined and added to `ITEM` and `ITEM_VARIATION` type catalog objects.
/// [Read more about custom
/// attributes](https://developer.squareup.com/docs/catalog-api/add-custom-attributes).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogCustomAttributeValue {
    /// The name of the custom attribute.
    pub name: Option<String>,
    /// The string value of the custom attribute. Populated if `type` = `STRING`.
    pub string_value: Option<String>,
    /// **Read-only.** The id of the [CatalogCustomAttributeDefinition] this value belongs to.
    pub custom_attribute_definition_id: Option<String>,
    /// **Read-only.** A copy of type from the associated `CatalogCustomAttributeDefinition`.
    pub r#type: Option<CatalogCustomAttributeDefinitionType>,
    /// Populated if `type` = `NUMBER`. Contains a string representation of a decimal number, using
    /// a `.` as the decimal separator.
    pub number_value: Option<String>,
    /// A `true` or `false` value. Populated if `type` = `BOOLEAN`.
    pub boolean_value: Option<bool>,
    /// One or more choices from `allowed_selections`. Populated if `type` = `SELECTION`.
    pub selection_uid_values: Option<Vec<String>>,
    /// **Read-only.** A copy of key from the associated `CatalogCustomAttributeDefinition`.
    pub key: Option<String>,
}
