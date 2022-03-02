//! Square environment in which to operate

const PRODUCTION_URL: &str = "https://connect.squareup.com";
const SANDBOX_URL: &str = "https://connect.squareupsandbox.com";

/// Identifies the Square environment in which this app instance is operating
#[derive(Clone, Debug)]
pub enum Environment {
    Production,
    Sandbox,
    Custom,
}

impl Environment {
    /// Gets the base Square API URL for this environment
    pub fn get_base_url(&self) -> Option<&'static str> {
        match self {
            Environment::Production => Some(PRODUCTION_URL),
            Environment::Sandbox => Some(SANDBOX_URL),
            Environment::Custom => None,
        }
    }
}

impl Default for Environment {
    /// Sandbox is the default environment
    fn default() -> Self {
        Self::Sandbox
    }
}
