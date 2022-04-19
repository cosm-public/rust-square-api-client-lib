//! Structs for configuring and making HTTP requests

mod http_client;
mod http_client_configuration;

pub use http_client::HttpClient;
pub use http_client_configuration::HttpClientConfiguration;
pub use http_client_configuration::RetryConfiguration;
