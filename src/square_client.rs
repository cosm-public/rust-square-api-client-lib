//! Gateway for the API

use std::collections::HashMap;

use crate::{api::*, config::Environment};

/// Gateway struct for the library.
/// This struct acts as a factory for Apis.
/// It holds the state of the SDK.
pub struct SquareClient {
    // pub mobile_authorization: MobileAuthorizationApi,
    // pub o_auth: OAuthApi,
    // pub apple_pay: ApplePayApi,
    // pub bank_accounts: BankAccountsApi,
    // pub bookings: BookingsApi,
    pub cards: CardsApi,
    // pub cash_drawers: CashDrawersApi,
    // pub catalog: CatalogApi,
    // pub customers: CustomersApi,
    // pub customer_groups: CustomerGroupsApi,
    // pub customer_segments: CustomerSegmentsApi,
    // pub devices: DevicesApi,
    // pub disputes: DisputesApi,
    // pub employees: EmployeesApi,
    // pub gift_cards: GiftCardsApi,
    // pub gift_card_activities: GiftCardActivitiesApi,
    // pub inventory: InventoryApi,
    // pub invoices: InvoicesApi,
    // pub labor: LaborApi,
    // pub locations: LocationsApi,
    // pub checkout: CheckoutApi,
    // pub transactions: TransactionsApi,
    // pub loyalty: LoyaltyApi,
    // pub merchants: MerchantsApi,
    // pub orders: OrdersApi,
    // pub payments: PaymentsApi,
    // pub refunds: RefundsApi,
    // pub sites: SitesApi,
    // pub snippets: SnippetsApi,
    // pub subscriptions: SubscriptionsApi,
    // pub team: TeamApi,
    // pub terminal: TerminalApi,
    // pub environment: Environment,
    // pub custom_url: String,
    // pub square_version: String,
    // pub http_client: HttpClient,
    // pub http_client_config: HttpClientConfiguration,
    // pub additional_headers: Headers,
    // pub user_agent_detail: String,
    // pub bearer_auth_manager: BearerAuthManager,
    // pub auth_managers: HashMap<String, AuthManager>,
    // pub http_callback: HttpCallback,
}

impl SquareClient {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for SquareClient {
    fn default() -> Self {
        Self {
            cards: CardsApi::default(),
        }
    }
}
