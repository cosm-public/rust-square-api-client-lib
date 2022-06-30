//! Programmatically catalogs a Square seller’s products for sale and services for hire.
//!
//! The Catalog API allows you to programmatically catalog products or services, including items,
//! variations, categories, discounts, taxes, modifiers, and more.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::ApiError, BatchDeleteCatalogObjectsRequest, BatchDeleteCatalogObjectsResponse,
        BatchRetrieveCatalogObjectsRequest, BatchRetrieveCatalogObjectsResponse,
        BatchUpsertCatalogObjectsRequest, BatchUpsertCatalogObjectsResponse, CatalogInfoResponse,
        CreateCatalogImageRequest, CreateCatalogImageResponse, DeleteCatalogObjectResponse,
        ListCatalogParameters, ListCatalogResponse, RetrieveCatalogObjectParameters,
        RetrieveCatalogObjectResponse, SearchCatalogItemsRequest, SearchCatalogItemsResponse,
        SearchCatalogObjectsRequest, SearchCatalogObjectsResponse, UpdateCatalogImageRequest,
        UpdateCatalogImageResponse, UpdateItemModifierListsRequest,
        UpdateItemModifierListsResponse, UpdateItemTaxesRequest, UpdateItemTaxesResponse,
        UpsertCatalogObjectRequest, UpsertCatalogObjectResponse,
    },
};

const DEFAULT_URI: &str = "/catalog";

/// Programmatically catalogs a Square seller’s products for sale and services for hire.
pub struct CatalogApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Catalog API endpoints
    client: HttpClient,
}

impl CatalogApi {
    /// Instantiates a new `CatalogApi`
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Deletes a set of [CatalogItem]s based on the provided list of target IDs and returns a set
    /// of successfully deleted IDs in the response.
    ///
    /// Deletion is a cascading event such that all children of the targeted object are also
    /// deleted. For example, deleting a CatalogItem will also delete all of its
    /// [CatalogItemVariation] children.
    ///
    /// `BatchDeleteCatalogObjects` succeeds even if only a portion of the targeted IDs can be
    /// deleted. The response will only include IDs that were actually deleted.
    pub async fn batch_delete_catalog_objects(
        &self,
        body: &BatchDeleteCatalogObjectsRequest,
    ) -> Result<BatchDeleteCatalogObjectsResponse, ApiError> {
        let url = format!("{}/batch-delete", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Returns a set of objects based on the provided ID.
    ///
    /// Each [CatalogItem] returned in the set includes all of its child information including: all
    /// of its [CatalogItemVariation] objects, references to its [CatalogModifierList] objects, and
    /// the ids of any [CatalogTax] objects that apply to it.
    pub async fn batch_retrieve_catalog_objects(
        &self,
        body: &BatchRetrieveCatalogObjectsRequest,
    ) -> Result<BatchRetrieveCatalogObjectsResponse, ApiError> {
        let url = format!("{}/batch-retrieve", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Creates or updates up to 10,000 target objects based on the provided list of objects.
    ///
    /// The target objects are grouped into batches and each batch is inserted/updated in an
    /// all-or-nothing manner. If an object within a batch is malformed in some way, or violates a
    /// database constraint, the entire batch containing that item will be disregarded. However,
    /// other batches in the same request may still succeed. Each batch may contain up to 1,000
    /// objects, and batches will be processed in order as long as the total object count for the
    /// request (items, variations, modifier lists, discounts, and taxes) is no more than 10,000.
    pub async fn batch_upsert_catalog_objects(
        &self,
        body: &BatchUpsertCatalogObjectsRequest,
    ) -> Result<BatchUpsertCatalogObjectsResponse, ApiError> {
        let url = format!("{}/batch-upsert", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Uploads an image file to be represented by a [CatalogImage] object that can be linked to an
    /// existing [CatalogObject] instance.
    ///
    /// The resulting `CatalogImage` is unattached to any `CatalogObject` if the object_id is not
    /// specified.
    ///
    /// This `CreateCatalogImage` endpoint accepts HTTP multipart/form-data requests with a JSON
    /// part and an image file part in JPEG, PJPEG, PNG, or GIF format. The maximum file size is
    /// 15MB.
    pub async fn create_catalog_image(
        &self,
        body: &CreateCatalogImageRequest,
        image_filepath: &str,
    ) -> Result<CreateCatalogImageResponse, ApiError> {
        let url = format!("{}/images", &self.url());
        let response = self.client.post_multipart(&url, body, image_filepath).await?;

        response.deserialize().await
    }

    /// Uploads a new image file to replace the existing one in the specified [CatalogImage] object.
    ///
    /// This `UpdateCatalogImage` endpoint accepts HTTP multipart/form-data requests with a JSON
    /// part and an image file part in JPEG, PJPEG, PNG, or GIF format. The maximum file size is
    /// 15MB.
    pub async fn update_catalog_image(
        &self,
        image_id: &str,
        body: &UpdateCatalogImageRequest,
        image_filepath: &str,
    ) -> Result<UpdateCatalogImageResponse, ApiError> {
        let url = format!("{}/images/{}", &self.url(), image_id);
        let response = self.client.put_multipart(&url, body, image_filepath).await?;

        response.deserialize().await
    }

    /// Retrieves information about the Square Catalog API, such as batch size limits that can be
    /// used by the `BatchUpsertCatalogObjects` endpoint.
    pub async fn catalog_info(&self) -> Result<CatalogInfoResponse, ApiError> {
        let url = format!("{}/info", &self.url());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Returns a list of all [CatalogObject]s of the specified types in the catalog.
    ///
    /// The `types` parameter is specified as a comma-separated list of the [CatalogObjectType]
    /// values, for example, "`ITEM`, `ITEM_VARIATION`, `MODIFIER`, `MODIFIER_LIST`, `CATEGORY`,
    /// `DISCOUNT`, `TAX`, `IMAGE`".
    ///
    /// **Important:** ListCatalog does not return deleted catalog items. To retrieve deleted
    /// catalog items, use [SearchCatalogObjects] and set the `include_deleted_objects` attribute
    /// value to `true`.
    pub async fn list_catalog(
        &self,
        params: &ListCatalogParameters,
    ) -> Result<ListCatalogResponse, ApiError> {
        let url = format!("{}/list{}", &self.url(), params.to_query_string());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Creates or updates the target [CatalogObject].
    pub async fn upsert_catalog_object(
        &self,
        body: &UpsertCatalogObjectRequest,
    ) -> Result<UpsertCatalogObjectResponse, ApiError> {
        let url = format!("{}/object", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Deletes a single [CatalogObject] based on the provided ID and returns the set of
    /// successfully deleted IDs in the response.
    ///
    /// Deletion is a cascading event such that all children of the targeted object are also
    /// deleted. For example, deleting a [CatalogItem] will also delete all of its
    /// [CatalogItemVariation] children.
    pub async fn delete_catalog_object(
        &self,
        object_id: &str,
    ) -> Result<DeleteCatalogObjectResponse, ApiError> {
        let url = format!("{}/object/{}", &self.url(), object_id);
        let response = self.client.delete(&url).await?;

        response.deserialize().await
    }

    /// Returns a single [CatalogItem] as a [CatalogObject] based on the provided ID.
    ///
    /// The returned object includes all of the relevant [CatalogItem] information including:
    /// [CatalogItemVariation] children, references to its [CatalogModifierList] objects, and the
    /// ids of any [CatalogTax] objects that apply to it.
    pub async fn retrieve_catalog_object(
        &self,
        object_id: &str,
        params: &RetrieveCatalogObjectParameters,
    ) -> Result<RetrieveCatalogObjectResponse, ApiError> {
        let url = format!("{}/object/{}{}", &self.url(), object_id, params.to_query_string());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Searches for [CatalogObject] of any type by matching supported search attribute values,
    /// excluding custom attribute values on items or item variations, against one or more of the
    /// specified query filters.
    ///
    /// This (`SearchCatalogObjects`) endpoint differs from the `SearchCatalogItems` endpoint in the
    /// following aspects:
    ///
    /// * `SearchCatalogItems` can only search for items or item variations, whereas
    ///   `SearchCatalogObjects` can search for any type of catalog objects.
    /// * `SearchCatalogItems` supports the custom attribute query filters to return items or item
    ///   variations that contain custom attribute values, where `SearchCatalogObjects` does not.
    /// * `SearchCatalogItems` does not support the include_deleted_objects filter to search for
    ///   deleted items or item variations, whereas `SearchCatalogObjects` does.
    /// * The both endpoints have different call conventions, including the query filter formats.
    pub async fn search_catalog_objects(
        &self,
        body: &SearchCatalogObjectsRequest,
    ) -> Result<SearchCatalogObjectsResponse, ApiError> {
        let url = format!("{}/search", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Searches for catalog items or item variations by matching supported search attribute values,
    /// including custom attribute values, against one or more of the specified query filters.
    ///
    /// This (`SearchCatalogItems`) endpoint differs from the `SearchCatalogObjects` endpoint in the
    /// following aspects:
    ///
    /// * `SearchCatalogItems` can only search for items or item variations, whereas
    ///   `SearchCatalogObjects` can search for any type of catalog objects.
    /// * `SearchCatalogItems` supports the custom attribute query filters to return items or item
    ///   variations that contain custom attribute values, where `SearchCatalogObjects` does not.
    /// * `SearchCatalogItems` does not support the include_deleted_objects filter to search for
    ///   deleted items or item variations, whereas `SearchCatalogObjects` does.
    /// * The both endpoints use different call conventions, including the query filter formats.
    pub async fn search_catalog_items(
        &self,
        body: &SearchCatalogItemsRequest,
    ) -> Result<SearchCatalogItemsResponse, ApiError> {
        let url = format!("{}/search-catalog-items", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Updates the [CatalogModifierList] objects that apply to the targeted [CatalogItem] without
    /// having to perform an upsert on the entire item.
    pub async fn update_item_modifier_lists(
        &self,
        body: &UpdateItemModifierListsRequest,
    ) -> Result<UpdateItemModifierListsResponse, ApiError> {
        let url = format!("{}/update-item-modifier-lists", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Updates the [CatalogTax] objects that apply to the targeted [CatalogItem] without having to
    /// perform an upsert on the entire item.
    pub async fn update_item_taxes(
        &self,
        body: &UpdateItemTaxesRequest,
    ) -> Result<UpdateItemTaxesResponse, ApiError> {
        let url = format!("{}/update-item-taxes", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
