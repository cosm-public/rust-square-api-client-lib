//! Model struct for RiskEvaluation type.

use serde::{Deserialize, Serialize};

use super::{enums::RiskEvaluationRiskLevel, DateTime};

/// Represents fraud risk information for the associated payment.
///
/// When you take a payment through Square's Payments API (using the `CreatePayment` endpoint),
/// Square evaluates it and assigns a risk level to the payment. Sellers can use this information to
/// determine the course of action (for example, provide the goods/services or refund the payment).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RiskEvaluation {
    /// Read only The timestamp when payment risk was evaluated.
    pub created_at: Option<DateTime>,
    /// The risk level associated with the payment.
    pub risk_level: Option<RiskEvaluationRiskLevel>,
}
