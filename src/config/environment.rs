//! Square environment in which to operate

const PRODUCTION_URL: &str = "https://connect.squareup.com";
const SANDBOX_URL: &str = "https://connect.squareupsandbox.com";

/// Identifies the Square environment in which this app instance is operating
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Environment {
    Production,
    Sandbox,
    Custom(String),
}

impl Environment {
    /// Gets the base Square API URL for this environment
    pub fn get_base_url(&self) -> String {
        match self {
            Environment::Production => String::from(PRODUCTION_URL),
            Environment::Sandbox => String::from(SANDBOX_URL),
            Environment::Custom(custom_url) => custom_url.clone(),
        }
    }
}

impl Default for Environment {
    /// Sandbox is the default environment
    fn default() -> Self {
        Self::Sandbox
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Environment;

    #[test]
    fn get_base_url_default() {
        let environment = Environment::default();
        assert_eq!(Environment::Sandbox, environment);
        assert_eq!(String::from("https://connect.squareupsandbox.com"), environment.get_base_url())
    }

    #[test]
    fn get_base_url_custom() {
        assert_eq!(
            String::from("some_custom_url"),
            Environment::Custom(String::from("some_custom_url")).get_base_url()
        );
    }

    #[test]
    fn get_base_url_production() {
        assert_eq!(
            String::from("https://connect.squareup.com"),
            Environment::Production.get_base_url()
        )
    }

    #[test]
    fn get_base_url_sandbox() {
        assert_eq!(
            String::from("https://connect.squareupsandbox.com"),
            Environment::Sandbox.get_base_url()
        )
    }
}
