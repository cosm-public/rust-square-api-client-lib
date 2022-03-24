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

use super::BaseApi;

const DEFAULT_URI: &str = "/customers";

pub struct CustomersApi {
    config: Configuration,
    client: HttpClient,
}

impl CustomersApi {
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Create a [customer]:
    /// You must provide at least one of the following values in your request to this endpoint:
    /// email_address, family_name, given_name, phone_number
    pub async fn create_customer(
        &self,
        body: &CreateCustomerRequest,
    ) -> Result<CreateCustomerResponse, ApiError> {
        let response = self.client.post(&self.url(), body).await?;

        self.handle_response(response).await
    }

    /// Retrieves a [Customer] by ID.
    pub async fn retrieve_customer(
        &self,
        customer_id: &str,
    ) -> Result<RetrieveCustomerResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), customer_id);
        let response = self.client.get(&url).await?;

        self.handle_response(response).await
    }

    /// Deletes a [Customer] by ID.
    pub async fn delete_customer(
        &self,
        customer_id: &str,
    ) -> Result<DeleteCustomerResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), customer_id);
        let response = self.client.delete(&url).await?;

        self.handle_response(response).await
    }

    /// Search Customers
    /// Searches the customer profiles associated with a Square account using a supported query filter.
    /// Calling SearchCustomers without any explicit query filter returns all customer profiles ordered
    /// alphabetically based on given_name and family_name.
    ///
    /// We can search customers on: email_address, phone_number, name and customer_id.
    pub async fn search_customers(
        &self,
        body: &SearchCustomersRequest,
    ) -> Result<SearchCustomersResponse, ApiError> {
        let url = format!("{}/search", &self.url());
        let response = self.client.post(&url, body).await?;

        self.handle_response(response).await
    }

    /// Retrieves a list of customers owned by the account making the request.
    /// A max of 25 customers will be returned.
    pub async fn list_customers(
        &self,
        params: &ListCustomersParameters,
    ) -> Result<ListCustomersResponse, ApiError> {
        let url = format!("{}{}", &self.url(), params.to_query_string());
        let response = self.client.get(&url).await?;

        self.handle_response(response).await
    }

    /// Updates Customer
    /// Currently the accepted fields for update are: email_address, phone_number, name
    pub async fn update_customer(
        &self,
        customer_id: &str,
        body: &UpdateCustomerRequest,
    ) -> Result<UpdateCustomerResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), customer_id);
        let response = self.client.put(&url, body).await?;

        self.handle_response(response).await
    }

    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}

impl BaseApi for CustomersApi {}
