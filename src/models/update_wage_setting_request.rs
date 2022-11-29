//! Request body struct for the Update Wage Setting API

use serde::Serialize;

use super::WageSetting;

/// This is a model struct for UpdateWageSettingRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateWageSettingRequest {
    /// The new `WageSetting` object that completely replaces the existing one.
    pub wage_setting: WageSetting,
}
