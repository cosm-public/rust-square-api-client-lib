//! Model struct for ProcessingFee type.

use serde::{Deserialize, Serialize};

use super::{enums::ProcessingFeeType, DateTime, Money};

/// Represents the Square processing fee.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProcessingFee {
    /// The timestamp of when the fee takes effect.
    pub effective_at: Option<DateTime>,
    /// The type of fee assessed or adjusted.
    pub r#type: Option<ProcessingFeeType>,
    /// The fee amount, which might be negative, that is assessed or adjusted by Square.
    ///
    /// Positive values represent funds being assessed, while negative values represent funds being
    /// returned.
    pub amount_money: Option<Money>,
}
