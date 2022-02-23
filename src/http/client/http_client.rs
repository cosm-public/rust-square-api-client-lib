// use super::HttpClientConfiguration;


// #[derive(Debug, Default)]
// pub struct HttpClient {
//     client: reqwest::Client,
// }

// impl HttpClient {
//     pub fn new(http_client_config: HttpClientConfiguration) -> Self {
//         match http_client_config.http_client {
//             Some(http_client) => {
//                 if http_client_config.override_http_client_configurations {
//                     apply_http_client_configurations(http_client, http_client_config)
//                 } else {
//                     Self {
//                         client: http_client.client,
//                     }
//                 }
//             },
//             None => {
//                 apply_http_client_configurations(default_client(), http_client_config)
//             },
//         }
//     }

//     fn apply_http_client_configurations(http_client: Self, http_client_config: HttpClientConfiguration) -> Self {
//         http_client.client
//     }
// }
