//! Get sales data for a Square seller, itemize payments, push orders to POS, and more.
//!
//! The Orders API is your one-stop shop for adding rich functionality to payments. You can itemize
//! payments using custom line items or catalog objects, send orders to physical Point of Sale
//! devices to be fulfilled, attach a customer to a payment, and more.
//!
//! In addition, the Orders API lets you search through all of a seller's past sales and returns
//! itemization data, customer references, and other details from sales made using POS or online.
//!
//! If you use the Square Orders API with a non-Square payments provider, Square charges a
//! transaction fee. For more information, see [Orders API fee
//! structure](https://developer.squareup.com/docs/payments-pricing#orders-api-fee-structure).

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::ApiError, BatchRetrieveOrdersRequest, BatchRetrieveOrdersResponse,
        CalculateOrderRequest, CalculateOrderResponse, CloneOrderRequest, CloneOrderResponse,
        CreateOrderRequest, CreateOrderResponse, PayOrderRequest, PayOrderResponse,
        RetrieveOrderResponse, SearchOrdersRequest, SearchOrdersResponse, UpdateOrderRequest,
        UpdateOrderResponse,
    },
};

const DEFAULT_URI: &str = "/orders";

/// Get sales data for a Square seller, itemize payments, push orders to POS, and more.
pub struct OrdersApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Orders API endpoints
    client: HttpClient,
}

impl OrdersApi {
    /// Instantiates a new `OrdersApi`
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Creates a new order that can include information about products for purchase and settings to apply to the purchase.
    ///
    /// To pay for a created order, see Pay for Orders.
    ///
    /// You can modify open orders using the UpdateOrder endpoint.
    pub async fn create_order(
        &self,
        body: &CreateOrderRequest,
    ) -> Result<CreateOrderResponse, ApiError> {
        let response = self.client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Retrieves a set of [orders](Order) by their IDs.
    ///
    /// If a given order ID does not exist, the ID is ignored instead of generating an error.
    pub async fn batch_retrieve_orders(
        &self,
        body: &BatchRetrieveOrdersRequest,
    ) -> Result<BatchRetrieveOrdersResponse, ApiError> {
        let url = format!("{}/batch-retrieve", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Enables applications to preview order pricing without creating an order.
    pub async fn calculate_order(
        &self,
        body: &CalculateOrderRequest,
    ) -> Result<CalculateOrderResponse, ApiError> {
        let url = format!("{}/calculate", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Creates a new order, in the `DRAFT` state, by duplicating an existing order.
    ///
    /// The newly created order has only the core fields (such as line items, taxes, and discounts)
    /// copied from the original order.
    pub async fn clone_order(
        &self,
        body: &CloneOrderRequest,
    ) -> Result<CloneOrderResponse, ApiError> {
        let url = format!("{}/clone", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Search all orders for one or more locations.
    ///
    /// Orders include all sales, returns, and exchanges regardless of how or when they entered the
    /// Square ecosystem (such as Point of Sale, Invoices, and Connect APIs).
    ///
    /// `SearchOrders` requests need to specify which locations to search and define a
    /// [SearchOrdersQuery] object that controls how to sort or filter the results. Your
    /// `SearchOrdersQuery` can:
    ///
    /// Set filter criteria. Set the sort order. Determine whether to return results as complete
    /// [Order] objects or as [OrderEntry] objects.
    ///
    /// Note that details for orders processed with Square Point of Sale while in offline mode might
    /// not be transmitted to Square for up to 72 hours. Offline orders have a `created_at` value
    /// that reflects the time the order was created, not the time it was subsequently transmitted
    /// to Square.
    pub async fn search_orders(
        &self,
        body: &SearchOrdersRequest,
    ) -> Result<SearchOrdersResponse, ApiError> {
        let url = format!("{}/search", &self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Retrieves an [Order] by ID.
    pub async fn retrieve_order(&self, order_id: &str) -> Result<RetrieveOrderResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), order_id);
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates an open [Order] by adding, replacing, or deleting fields.
    ///
    /// Orders with a `COMPLETED` or `CANCELED` state cannot be updated.
    ///
    /// An `UpdateOrder` request requires the following:
    ///
    /// * The `order_id` in the endpoint path, identifying the order to update.
    /// * The latest `version` of the order to update.
    /// * The [sparse order](https://developer.squareup.com/docs/orders-api/manage-orders#sparse-order-objects)
    ///   containing only the fields to update and the version to which the update is being applied.
    /// * If deleting fields, the [dot notation
    ///   paths](https://developer.squareup.com/docs/orders-api/manage-orders#on-dot-notation)
    ///   identifying the fields to clear.
    ///
    /// To pay for an order, see Pay for Orders.
    pub async fn update_order(
        &self,
        order_id: &str,
        body: &UpdateOrderRequest,
    ) -> Result<UpdateOrderResponse, ApiError> {
        let url = format!("{}/{}", &self.url(), order_id);
        let response = self.client.put(&url, body).await?;

        response.deserialize().await
    }

    /// Pay for an [Order] using one or more approved [Payment]s or settle an order with a total of
    /// `0`.
    ///
    /// The total of the `payment_ids` listed in the request must be equal to the order total.
    /// Orders with a total amount of `0` can be marked as paid by specifying an empty array of
    /// `payment_ids` in the request.
    ///
    /// To be used with `PayOrder`, a payment must:
    ///
    /// * Reference the order by specifying the `order_id` when creating the payment. Any approved
    ///   payments that reference the same `order_id` not specified in the `payment_ids` is
    ///   canceled.
    /// * Be approved with [delayed
    ///   capture](https://developer.squareup.com/docs/payments-api/take-payments#delayed-capture).
    ///   Using a delayed capture payment with `PayOrder` completes the approved payment.
    pub async fn pay_order(
        &self,
        order_id: &str,
        body: &PayOrderRequest,
    ) -> Result<PayOrderResponse, ApiError> {
        let url = format!("{}/{}/pay", &self.url(), order_id);
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
