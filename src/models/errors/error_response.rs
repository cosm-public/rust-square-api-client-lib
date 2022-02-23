use serde::Deserialize;

use super::Error;


#[derive(Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<Error>,
}
