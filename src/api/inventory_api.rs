//!The Inventory API exposes a set of objects to represent inventory adjustments
//! and physical counts for quantities of products (as item variations) and
//! transitions of stocked products to the relevant inventory state.
//!
//! hese include the following key data types for the Inventory API:

//! InventoryCount - computed quantity of an item variation at a specific location.
//! InventoryAdjustment - quantity of an item variation transitioning from one state to another.
//! InventoryPhysicalCount - verified quantity of an item variation at a specific location.
//! SourceApplication - application that makes an inventory change and helps trace sources of inventory changes.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::ApiError, BatchChangeInventoryRequest, BatchChangeInventoryResponse,
        BatchRetrieveInventoryChangesRequest, BatchRetrieveInventoryChangesResponse,
        BatchRetrieveInventoryCountsRequest, BatchRetrieveInventoryCountsResponse,
        RetrieveInventoryAdjustmentResponse, RetrieveInventoryCountParams,
        RetrieveInventoryCountResponse, RetrieveInventoryPhysicalCount,
        RetrieveInventoryTransferResponse,
    },
};

const DEFAULT_URI: &str = "/inventory";

pub struct InventoryApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Inventory API endpoints
    client: HttpClient,
}

impl InventoryApi {
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Retrieves the current calculated stock count for a given CatalogObject at a given set of Locations.
    pub async fn retrieve_inventory_count(
        &self,
        catalog_object_id: &str,
        params: RetrieveInventoryCountParams,
    ) -> Result<RetrieveInventoryCountResponse, ApiError> {
        let url = format!("{}/{}{}", &self.url(), catalog_object_id, params.to_query_string());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Returns the InventoryTransfer object with the provided transfer_id.
    pub async fn retrieve_inventory_transfer(
        &self,
        transfer_id: &str,
    ) -> Result<RetrieveInventoryTransferResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), transfer_id);
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Returns the InventoryAdjustment object with the provided adjustment id.
    pub async fn retrieve_inventory_adjustment(
        &self,
        adjustment_id: &str,
    ) -> Result<RetrieveInventoryAdjustmentResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), adjustment_id);
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Returns the InventoryPhysicalCount object with the provided physical_count_id.
    pub async fn retrieve_inventory_physical_count(
        &self,
        physical_count_id: &str,
    ) -> Result<RetrieveInventoryPhysicalCount, ApiError> {
        let url = format!("{}/{}", &self.url(), physical_count_id);
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Applies adjustments and counts to the provided item quantities.
    /// On success: returns the current calculated counts for all objects referenced in the request.
    /// On failure: returns a list of related errors.
    pub async fn batch_change(
        &self,
        body: &BatchChangeInventoryRequest,
    ) -> Result<BatchChangeInventoryResponse, ApiError> {
        let url = format!("{}/changes/batch-create", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Returns current counts for the provided CatalogObjects at the requested Locations.
    /// Results are paginated and sorted in descending order according to their calculated_at timestamp.
    pub async fn batch_retrieve_count(
        &self,
        body: &BatchRetrieveInventoryCountsRequest,
    ) -> Result<BatchRetrieveInventoryCountsResponse, ApiError> {
        let url = format!("{}/counts/batch-retrieve", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Returns historical physical counts and adjustments based on the provided filter criteria.
    /// Results are paginated and sorted in ascending order according their occurred_at timestamp.
    pub async fn batch_retrieve_changes(
        &self,
        body: &BatchRetrieveInventoryChangesRequest,
    ) -> Result<BatchRetrieveInventoryChangesResponse, ApiError> {
        let url = format!("{}/changes/batch-retrieve", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
