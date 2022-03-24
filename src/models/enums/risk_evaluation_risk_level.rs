//! Model for RiskEvaluationRiskLevel enum.

use serde::{Deserialize, Serialize};

/// The risk level associated with a payment.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum RiskEvaluationRiskLevel {
    /// Indicates Square is still evaluating the payment.
    Pending,
    /// Indicates payment risk is within the normal range.
    Normal,
    /// Indicates elevated risk level associated with the payment.
    Moderate,
    /// Indicates significantly elevated risk level with the payment.
    High,
}
