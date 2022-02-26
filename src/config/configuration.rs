// use crate::http::client::HttpClientConfiguration;

use super::Environment;


/// Configuration struct for the library
pub struct Configuration {
    /// Current API environment
    pub environment: Environment,
    /// Base URL requests are made to. Defaults to `https://connect.squareup.com`
    pub custom_url: String,
    /// Square connect API versions.
    pub square_version: String,
    /// Http Client Configuration instance.
    //pub http_client_config: HttpClientConfiguration,
    /// Additional headers to add to each API request.
    // pub additional_headers: Headers,
    /// Additional detail which can be appended with User-Agent header.
    pub user_agent_detail: String,
    /// The timeout to use for making HTTP requests.
    pub timeout: i32,
    /// OAuth 2.0 Access Token
    pub access_token: String,
    /// Base URI
    pub base_uri: String,
}


