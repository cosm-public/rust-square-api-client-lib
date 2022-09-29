//! Response struct for the Catalog Info API

use serde::Deserialize;

use super::{errors::Error, CatalogInfoResponseLimits, StandardUnitDescriptionGroup};

/// This is a model struct for CatalogInfoResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CatalogInfoResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// Limits that apply to this API.
    pub limits: Option<CatalogInfoResponseLimits>,
    /// Names and abbreviations for standard units.
    pub standard_unit_description_group: Option<StandardUnitDescriptionGroup>,
}
