# Square API Client

Square API Client provides a Rust wrapper on top of the Square web APIs.

## Square API

Square APIs enable you to accept payments securely and integrate your app with Square’s first party
product ecosystem. Build full-featured business apps for yourself or millions of Square sellers.

[The Square API Reference](https://developer.squareup.com/reference/square) is organized around core
business workflows: taking payments, managing orders, syncing items and inventory with Square Point
of Sale, creating customer records, managing business locations, and enabling Square sellers to use
your app.

## Usage

### Setting up the client

The client is instantiated most simply with
```rust
use square_api_client::{config::Configuration, SquareClient};

let client = SquareClient::try_new(Configuration::default()).unwrap();
```

For this to work, it's necessary to set an environment variable with the name of `SQUARE_API_TOKEN`
and the value of your API Token string... otherwise, you'll get API errors when making live calls.

Other default values are `Sandbox` for the Square environment, `2022-02-16` API version, a base URI
of `/v2`, a timeout of 60 seconds and no HTTP Client retries.

The Square client can be customized a bit via the properties shown here:
```rust
use square_api_client::{
    config::{Configuration, Environment},
    http::{client::{HttpClientConfiguration, RetryConfiguration}, Headers},
};
use std::time::Duration;

let mut headers = Headers::default();
headers.set_user_agent("Some User Agent String");
headers.insert("X_SOME_CUSTOM_HEADER", "custom_header_value");

let client = SquareClient::try_new(Configuration {
    environment: Environment::Production,
    square_version: String::from("2022-09-21"),
    http_client_config: HttpClientConfiguration {
        timeout: 30,
        user_agent: String::from("Some User Agent String"),
        default_headers: headers,
        retry_configuration: RetryConfiguration {
            retries_count: 1,
            min_retry_interval: Duration::from_secs(1),
            max_retry_interval: Duration::from_secs(30 * 60),
            backoff_exponent: 3,
        },
    },
    access_token: String::from("Bearer MY_ACCESS_TOKEN"),
    base_uri: String::from("/v2"),
}).unwrap();
```

### Using the client

Once you have a `SquareClient` instance access to the various APIs is through its properties.
For example, to access the Payments API, you would:
```rust
use square_api_client::{
    config::Configuration,
    models::CreatePaymentRequest,
    SquareClient,
};

let client = SquareClient::try_new(Configuration::default()).unwrap();
let request = CreatePaymentRequest::default(); // this actually has many fields to fill
let response = client.payments_api.create_payment(request).await.unwrap();
```

## Progress

The intent is to wrap all of the Square APIs in this crate. So far, it includes some of the more
commonly required features.

### Implemented so far

So far, we have the following APIs wrapped in the Rust Square API Client:
- [Cards](https://developer.squareup.com/reference/square/cards-api)
- [Catalog](https://developer.squareup.com/reference/square/catalog-api)
- [Customers](https://developer.squareup.com/reference/square/customers-api)
- [Gift Cards](https://developer.squareup.com/reference/square/gift-cards-api)
- [Inventory](https://developer.squareup.com/reference/square/inventory-api)
- [Locations](https://developer.squareup.com/reference/square/locations-api)
- [Orders](https://developer.squareup.com/reference/square/orders-api)
- [Payments](https://developer.squareup.com/reference/square/payments-api)
- [Refunds](https://developer.squareup.com/reference/square/refunds-api)

### To be implemented

Future versions of this crate will implement the following APIs:
- [Apple Pay](https://developer.squareup.com/reference/square/apple-pay-api)
- [Bank Accounts](https://developer.squareup.com/reference/square/bank-accounts-api)
- [Bookings](https://developer.squareup.com/reference/square/bookings-api)
- [Cash Drawers](https://developer.squareup.com/reference/square/cash-drawers-api)
- [Checkout](https://developer.squareup.com/reference/square/checkout-api)
- [Customer Custom Attributes](https://developer.squareup.com/reference/square/customer-custom-attributes-api)
- [Customer Groups](https://developer.squareup.com/reference/square/customer-groups-api)
- [Customer Segments](https://developer.squareup.com/reference/square/customer-segments-api)
- [Devices](https://developer.squareup.com/reference/square/devices-api)
- [Disputes](https://developer.squareup.com/reference/square/disputes-api)
- [Gift Card Activities](https://developer.squareup.com/reference/square/gift-card-activities-api)
- [Invoices](https://developer.squareup.com/reference/square/invoices-api)
- [Labor](https://developer.squareup.com/reference/square/labor-api)
- [Loyalty](https://developer.squareup.com/reference/square/loyalty-api)
- [Merchants](https://developer.squareup.com/reference/square/merchants-api)
- [Mobile Authorization](https://developer.squareup.com/reference/square/mobile-authorization-api)
- [OAuth](https://developer.squareup.com/reference/square/oauth-api)
- [Payouts](https://developer.squareup.com/reference/square/payouts-api)
- [Sites](https://developer.squareup.com/reference/square/sites-api)
- [Snippets](https://developer.squareup.com/reference/square/snippets-api)
- [Subscriptions](https://developer.squareup.com/reference/square/subscriptions-api)
- [Team](https://developer.squareup.com/reference/square/team-api)
- [Terminal](https://developer.squareup.com/reference/square/terminal-api)
- [Vendors](https://developer.squareup.com/reference/square/vendors-api)
- [Webhook Subscriptions](https://developer.squareup.com/reference/square/webhook-subscriptions-api)

