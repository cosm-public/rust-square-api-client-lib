//! Retrieve customer segments (also called smart groups) in a business account.
//!
//! The Customer Segments API lets you retrieve information about the segments defined for a
//! business. Square sellers can create customer segments in the Seller Dashboard or Point of Sale
//! by defining filters for the segment. For example, a segment can include customers who have
//! visited more than 10 times. Customers are automatically added to and removed from the segment
//! over time based on this criterion.
//!
//! You can inspect the customer's `segment_ids` property to determine which segments a customer
//! belongs to. Then, you can use the Customer Segments API to retrieve basic details about each
//! segment, such as the segment name and the time when it was created.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::ApiError, ListCustomerSegmentsParameters, ListCustomerSegmentsResponse,
        RetrieveCustomerSegmentResponse,
    },
};

const DEFAULT_URI: &str = "/customers/segments";

/// Retrieve customer segments (also called smart groups) in a business account.
pub struct CustomerSegmentsApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Customer Segments API endpoints
    client: HttpClient,
}

impl CustomerSegmentsApi {
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Retrieves the list of customer segments of a business.
    pub async fn list_customer_segments(
        &self,
        params: &ListCustomerSegmentsParameters,
    ) -> Result<ListCustomerSegmentsResponse, ApiError> {
        let url = format!("{}{}", &self.url(), params.to_query_string());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Retrieves a specific customer segment as identified by the `segment_id` value.
    pub async fn retrieve_customer_segment(
        &self,
        segment_id: &str,
    ) -> Result<RetrieveCustomerSegmentResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), segment_id);
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
