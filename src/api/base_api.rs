use crate::{models::errors::ApiError, http::client::HttpClient};


pub(crate) trait BaseApi {
    fn get_client(&self) -> Result<HttpClient, ApiError> {
        HttpClient::try_new(None)
    }
}
