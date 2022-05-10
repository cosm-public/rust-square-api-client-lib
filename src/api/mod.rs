mod base_api;
pub(crate) use base_api::BaseApi;

mod cards_api;
mod catalog_api;
mod customers_api;
mod orders_api;
mod payments_api;

pub use cards_api::CardsApi;
pub use catalog_api::CatalogApi;
pub use customers_api::CustomersApi;
pub use orders_api::OrdersApi;
pub use payments_api::PaymentsApi;
