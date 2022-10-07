//! Represents a collection of Request Headers

use super::client::HttpClientConfiguration;
use crate::config;
use crate::models::errors::ApiError;
use log::error;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;

/// A collection of Request Headers
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Headers {
    /// The request headers as key-value pairs
    pub headers: HashMap<String, String>,
}

impl Headers {
    /// Indicates whether the headers include the User Agent header
    pub fn has_user_agent(&self) -> bool {
        self.headers.contains_key("user-agent")
    }

    /// Sets the User Agent header
    pub fn set_user_agent(&mut self, user_agent: &str) -> Option<String> {
        self.insert("user-agent", user_agent)
    }

    /// Adds a request header to the collection
    pub fn insert(&mut self, header_name: &str, header_value: &str) -> Option<String> {
        self.headers.insert(String::from(header_name), String::from(header_value))
    }
}

impl Default for Headers {
    /// The default set of request headers
    /// * Content-Type: application/json
    /// * Square-Version
    /// * accept: application/json
    /// * user-agent
    /// * Authorization
    fn default() -> Self {
        let mut headers = HashMap::new();

        headers.insert(String::from("Content-Type"), String::from("application/json"));
        headers
            .insert(String::from("Square-Version"), String::from(config::DEFAULT_SQUARE_VERSION));
        headers.insert(String::from("accept"), String::from("application/json"));
        headers.insert(String::from("user-agent"), HttpClientConfiguration::default_user_agent());
        headers.insert(String::from("Authorization"), config::default_authorization());

        Self { headers }
    }
}

impl TryFrom<&Headers> for HeaderMap {
    type Error = ApiError;

    /// Converts `Headers` into Reqwest lib's `HeaderMap`
    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        let mut header_map = Self::new();
        for (k, v) in &headers.headers {
            let header_name = HeaderName::from_bytes(k.as_bytes()).map_err(|e| {
                let msg = format!("Error generating {} header name: {}", k, e);
                error!("{}", msg);
                ApiError::new(&msg)
            })?;
            let header_value = HeaderValue::from_bytes(v.as_bytes()).map_err(|e| {
                let msg = format!("Error generating {} header value for header {}: {}", v, k, e);
                error!("{}", msg);
                ApiError::new(&msg)
            })?;
            header_map.insert(header_name, header_value);
        }

        Ok(header_map)
    }
}

#[cfg(test)]
mod tests {
    use crate::config;
    use crate::http::client::HttpClientConfiguration;
    use crate::http::Headers;
    use reqwest::header::HeaderMap;
    use std::collections::HashMap;

    #[test]
    fn headers_default() {
        let headers = Headers::default();
        assert_eq!(headers.headers.get("Content-Type"), Some(&String::from("application/json")));
        assert_eq!(headers.headers.get("Square-Version"), Some(&String::from("2022-02-16")));
        assert_eq!(headers.headers.get("accept"), Some(&String::from("application/json")));
        assert_eq!(
            headers.headers.get("user-agent"),
            Some(&HttpClientConfiguration::default_user_agent())
        );
        assert_eq!(headers.headers.get("Authorization"), Some(&config::default_authorization()));
        assert!(headers.has_user_agent());
    }

    #[test]
    fn headers_has_user_agent() {
        let mut headers = Headers::default();
        headers.headers = HashMap::new();
        assert!(!headers.has_user_agent());
        headers.set_user_agent("some-user-agent");
        assert!(headers.has_user_agent());
    }

    #[test]
    fn headers_set_user_agent() {
        let mut headers = Headers::default();
        assert!(!(headers.headers.get("user-agent") == Some(&String::from("some-user-agent"))));
        headers.set_user_agent("some-user-agent");
        assert_eq!(Some(&String::from("some-user-agent")), headers.headers.get("user-agent"));
    }

    #[test]
    fn try_from_ok() {
        assert!(HeaderMap::try_from(&Headers::default()).is_ok());
    }

    #[test]
    fn try_from_error() {
        let mut headers = Headers::default();
        headers.headers = HashMap::new();
        headers
            .headers
            .insert(String::from("some_faulty_code\u{1234}"), String::from("some_value"));
        assert!(HeaderMap::try_from(&headers).is_err());
    }
}
