//! Model struct for CatalogQuickAmountsSettings type.

use serde::{Deserialize, Serialize};

use super::{enums::CatalogQuickAmountsSettingsOption, CatalogQuickAmount};

/// A parent Catalog Object model represents a set of Quick Amounts and the settings control the
/// amounts.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogQuickAmountsSettings {
    /// Represents the option seller currently uses on Quick Amounts.
    pub option: CatalogQuickAmountsSettingsOption,
    /// Represents location's eligibility for auto amounts The boolean should be consistent with
    /// whether there are AUTO amounts in the `amounts`.
    pub eligible_for_auto_amounts: Option<bool>,
    /// Represents a set of Quick Amounts at this location.
    pub amounts: Option<Vec<CatalogQuickAmount>>,
}
