//! Error model structs

mod api_error;
pub use api_error::ApiError;

mod error;
pub use error::Error;

mod error_response;
pub use error_response::ErrorResponse;
