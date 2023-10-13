/// Model struct for QuickPay type
use serde::{Deserialize, Serialize};

use super::Money;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct QuickPay {
    location_id: String,
    name: String,
    price_money: Money,
}
