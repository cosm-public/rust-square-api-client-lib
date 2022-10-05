//! Create and manage customer profiles and sync CRM systems with Square.
//!
//! The Customers API enables you to create and manage customer profiles, as well as search for
//! customers based on various criteria (including customer group membership). You can also use the
//! API to sync contacts between your CRM system and Square.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::ApiError, CreateCustomerRequest, CreateCustomerResponse, DeleteCustomerResponse,
        ListCustomersParameters, ListCustomersResponse, RetrieveCustomerResponse,
        SearchCustomersRequest, SearchCustomersResponse, UpdateCustomerRequest,
        UpdateCustomerResponse,
    },
};

const DEFAULT_URI: &str = "/customers";

/// Create and manage [Customer] profiles and sync CRM systems with Square.
pub struct CustomersApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Customers API endpoints
    client: HttpClient,
}

impl CustomersApi {
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Creates a new [Customer] for a business.
    ///
    /// You must provide at least one of the following values in your request to this endpoint:
    ///
    /// * `given_name`
    /// * `family_name`
    /// * `company_name`
    /// * `email_address`
    /// * `phone_number`
    pub async fn create_customer(
        &self,
        body: &CreateCustomerRequest,
    ) -> Result<CreateCustomerResponse, ApiError> {
        let response = self.client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Returns details for a single [Customer].
    pub async fn retrieve_customer(
        &self,
        customer_id: &str,
    ) -> Result<RetrieveCustomerResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), customer_id);
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Deletes a [Customer] profile from a business.
    ///
    /// This operation also unlinks any associated cards on file.
    ///
    /// As a best practice, you should include the version field in the request to enable optimistic
    /// concurrency control. The value must be set to the current version of the customer profile.
    ///
    /// To delete a customer profile that was created by merging existing profiles, you must use the
    /// ID of the newly created profile.
    pub async fn delete_customer(
        &self,
        customer_id: &str,
    ) -> Result<DeleteCustomerResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), customer_id);
        let response = self.client.delete(&url).await?;

        response.deserialize().await
    }

    /// Searches the [Customer] profiles associated with a Square account using a supported query
    /// filter.
    ///
    /// Calling `SearchCustomers` without any explicit query filter returns all customer profiles
    /// ordered alphabetically based on `given_name` and `family_name`.
    ///
    /// Under normal operating conditions, newly created or updated customer profiles become
    /// available for the search operation in well under 30 seconds. Occasionally, propagation of
    /// the new or updated profiles can take closer to one minute or longer, especially during
    /// network incidents and outages.
    pub async fn search_customers(
        &self,
        body: &SearchCustomersRequest,
    ) -> Result<SearchCustomersResponse, ApiError> {
        let url = format!("{}/search", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Lists [Customer] profiles associated with a Square account.
    ///
    /// Under normal operating conditions, newly created or updated customer profiles become
    /// available for the listing operation in well under 30 seconds. Occasionally, propagation of
    /// the new or updated profiles can take closer to one minute or longer, especially during
    /// network incidents and outages.
    pub async fn list_customers(
        &self,
        params: &ListCustomersParameters,
    ) -> Result<ListCustomersResponse, ApiError> {
        let url = format!("{}{}", &self.url(), params.to_query_string());
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates a [Customer] profile.
    ///
    /// To change an attribute, specify the new value. To remove an attribute, specify the value as
    /// an empty string or empty object.
    ///
    /// As a best practice, you should include the `version` field in the request to enable
    /// [optimistic
    /// concurrency](https://developer.squareup.com/docs/working-with-apis/optimistic-concurrency)
    /// control. The value must be set to the current version of the customer profile.
    ///
    /// To update a customer profile that was created by merging existing profiles, you must use the
    /// ID of the newly created profile.
    ///
    /// You cannot use this endpoint to change cards on file. To make changes, use the [Cards
    /// API](https://developer.squareup.com/reference/square/cards-api) or [Gift Cards
    /// API](https://developer.squareup.com/reference/square/giftcards-api).
    pub async fn update_customer(
        &self,
        customer_id: &str,
        body: &UpdateCustomerRequest,
    ) -> Result<UpdateCustomerResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), customer_id);
        let response = self.client.put(&url, body).await?;

        response.deserialize().await
    }

    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
