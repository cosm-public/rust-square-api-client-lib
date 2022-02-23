//! Model struct for Money type

use serde::Deserialize;

/// This is a model struct for Money type.
#[derive(Debug, Default, Deserialize, Hash, PartialEq)]
pub struct Money {
    /// The amount of money, in the smallest denomination of the currency indicated by `currency`.
    /// For example, when `currency` is `USD`, `amount` is in cents. Monetary amounts can be
    /// positive or negative. See the specific field description to determine the meaning of the
    /// sign in a particular case.
    pub amount: i32,
    /// Indicates the associated currency for an amount of money. Values correspond to [ISO
    /// 4217](https://wikipedia.org/wiki/ISO_4217).
    pub currency: String,
}
