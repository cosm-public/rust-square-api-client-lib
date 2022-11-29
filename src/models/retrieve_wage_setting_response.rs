//! Response body struct for the Retrieve Wage Setting API

use serde::Deserialize;

use super::{errors::Error, WageSetting};

/// This is a model struct for RetrieveWageSettingResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RetrieveWageSettingResponse {
    /// The successfully retrieved `WageSetting` object.
    pub wage_setting: Option<WageSetting>,
    /// The errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
