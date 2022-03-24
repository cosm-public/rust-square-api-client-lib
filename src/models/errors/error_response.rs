use serde::Deserialize;

use super::Error;

#[derive(Clone, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<Error>,
}
