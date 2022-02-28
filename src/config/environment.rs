
const PRODUCTION_URL: &str = "https://connect.squareup.com";
const SANDBOX_URL: &str = "https://connect.squareupsandbox.com";

#[derive(Clone, Debug)]
pub enum Environment {
    Production,
    Sandbox,
    Custom,
}

impl Environment {
    pub fn get_base_url(&self) -> Option<&'static str> {
        match self {
            Environment::Production => Some(PRODUCTION_URL),
            Environment::Sandbox => Some(SANDBOX_URL),
            Environment::Custom => None,
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::Sandbox
    }
}
