use super::Error;

#[derive(Clone, Debug, Default)]
pub struct ApiError {
    pub message: String,
    pub errors: Vec<Error>,
}

impl ApiError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            ..Default::default()
        }
    }

    pub fn with_response_errors(message: &str, errors: &[Error]) -> Self {
        Self {
            message: message.to_owned(),
            errors: errors.to_vec(),
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API Error: {:?}", self)
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
