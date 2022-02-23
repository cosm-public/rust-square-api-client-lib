// use std::collections::HashSet;

// use crate::http::request::HttpMethod;

// use super::HttpClient;

// const DEFAULT_TIMEOUT: i32 = 60;

// #[derive(Debug)]
// pub struct HttpClientConfiguration {
//     pub timeout: i32,
//     pub number_of_retries: i16,
//     pub back_off_factor: i16,
//     pub retry_interval: i32,
//     pub http_status_codes_to_retry: HashSet<i16>,
//     pub http_methods_to_retry: HashSet<HttpMethod>,
//     pub maximum_retry_wait_time: i32,
//     pub should_retry_on_timeout: bool,
//     pub http_client: Option<HttpClient>,
//     pub override_http_client_configurations: bool,
// }

// impl Default for HttpClientConfiguration {
//     fn default() -> Self {
//         Self {
//             timeout: DEFAULT_TIMEOUT,
//             ..Default::default()
//         }
//     }
// }
