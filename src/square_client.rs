//! Gateway for the API

use crate::{api::*, config::Configuration, http::client::HttpClient, models::errors::ApiError};

/// Gateway struct for the library.
/// This struct acts as a factory for Apis.
/// It holds the state of the SDK.
pub struct SquareClient {
    // pub mobile_authorization: MobileAuthorizationApi,
    // pub o_auth: OAuthApi,
    pub apple_pay: ApplePayApi,
    // pub bank_accounts: BankAccountsApi,
    // pub bookings: BookingsApi,
    pub cards: CardsApi,
    // pub cash_drawers: CashDrawersApi,
    pub catalog: CatalogApi,
    pub customers: CustomersApi,
    pub customer_groups: CustomerGroupsApi,
    pub customer_segments: CustomerSegmentsApi,
    // pub devices: DevicesApi,
    // pub disputes: DisputesApi,
    // pub employees: EmployeesApi,
    pub gift_cards: GiftCardsApi,
    pub gift_card_activities: GiftCardActivitiesApi,
    pub inventory: InventoryApi,
    pub invoices: InvoicesApi,
    // pub labor: LaborApi,
    pub locations: LocationsApi,
    // pub checkout: CheckoutApi,
    // pub transactions: TransactionsApi,
    // pub loyalty: LoyaltyApi,
    // pub merchants: MerchantsApi,
    pub orders: OrdersApi,
    pub payments: PaymentsApi,
    pub refunds: RefundsApi,
    // pub sites: SitesApi,
    // pub snippets: SnippetsApi,
    pub subscriptions: SubscriptionsApi,
    pub team: TeamApi,
    // pub terminal: TerminalApi,
    // pub config: Configuration,
    // pub http_client: HttpClient,
    // pub http_client_config: HttpClientConfiguration,
    // pub additional_headers: Headers,
    // pub user_agent_detail: String,
    // pub bearer_auth_manager: BearerAuthManager,
    // pub auth_managers: HashMap<String, AuthManager>,
    // pub http_callback: HttpCallback,
}

impl SquareClient {
    pub fn try_new(config: Configuration) -> Result<Self, ApiError> {
        let http_client = HttpClient::try_new(&config.http_client_config)?;

        let this: SquareClient = Self {
            apple_pay: ApplePayApi::new(config.clone(), http_client.clone()),
            cards: CardsApi::new(config.clone(), http_client.clone()),
            catalog: CatalogApi::new(config.clone(), http_client.clone()),
            customer_groups: CustomerGroupsApi::new(config.clone(), http_client.clone()),
            customer_segments: CustomerSegmentsApi::new(config.clone(), http_client.clone()),
            customers: CustomersApi::new(config.clone(), http_client.clone()),
            gift_card_activities: GiftCardActivitiesApi::new(config.clone(), http_client.clone()),
            gift_cards: GiftCardsApi::new(config.clone(), http_client.clone()),
            inventory: InventoryApi::new(config.clone(), http_client.clone()),
            invoices: InvoicesApi::new(config.clone(), http_client.clone()),
            locations: LocationsApi::new(config.clone(), http_client.clone()),
            orders: OrdersApi::new(config.clone(), http_client.clone()),
            payments: PaymentsApi::new(config.clone(), http_client.clone()),
            refunds: RefundsApi::new(config.clone(), http_client.clone()),
            subscriptions: SubscriptionsApi::new(config.clone(), http_client.clone()),
            team: TeamApi::new(config, http_client),
        };

        Ok(this)
    }
}
