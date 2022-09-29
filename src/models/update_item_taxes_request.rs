//! Request struct for the Update Item Taxes API

use serde::Serialize;

/// This is a model struct for UpdateItemTaxesRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateItemTaxesRequest {
    /// IDs for the CatalogItems associated with the CatalogTax objects being updated. No more than
    /// 1,000 IDs may be provided.
    pub item_ids: Vec<String>,
    /// IDs of the CatalogTax objects to enable. At least one of `taxes_to_enable` or
    /// `taxes_to_disable` must be specified.
    pub taxes_to_enable: Option<Vec<String>>,
    /// IDs of the CatalogTax objects to disable. At least one of `taxes_to_enable` or
    /// `taxes_to_disable` must be specified.
    pub taxes_to_disable: Option<Vec<String>>,
}
