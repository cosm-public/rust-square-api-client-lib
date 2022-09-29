//! Response struct for the Update Item Modifier Lists API

use serde::Deserialize;

use super::{errors::Error, DateTime};

/// This is a model struct for UpdateItemModifierListsResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct UpdateItemModifierListsResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The database timestamp of this update
    pub updated_at: Option<DateTime>,
}
