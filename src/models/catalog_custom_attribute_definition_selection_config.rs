//! Model struct for CatalogCustomAttributeDefinitionSelectionConfig type.

use serde::{Deserialize, Serialize};

use super::CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection;

/// Configuration associated with `SELECTION`-type custom attribute definitions.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogCustomAttributeDefinitionSelectionConfig {
    /// The maximum number of selections that can be set. The maximum value for this attribute is
    /// 100. The default value is 1. The value can be modified, but changing the value will not
    /// affect existing custom attribute values on objects. Clients need to handle custom attributes
    /// with more selected values than allowed by this limit.
    max_allowed_selections: Option<i32>,
    /// The set of valid `CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection`.
    /// Up to a maximum of 100 selections can be defined. Can be modified.
    allowed_selections:
        Option<Vec<CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection>>,
}
