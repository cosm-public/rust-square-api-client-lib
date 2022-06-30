//! Model struct for CreateLocationRequest type

use serde::Serialize;

use super::Location;

/// This is a model struct for CreateLocationRequest type
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct CreateLocationRequest {
    /// The initial values of the location being created. The name field is required and must be
    /// unique within a seller account. All other fields are optional, but any information you care
    /// about for the location should be included. The remaining fields are automatically added
    /// based on the data from the main location.
    pub location: Location,
}
