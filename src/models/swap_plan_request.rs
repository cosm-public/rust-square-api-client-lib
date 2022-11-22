//! Request body struct for the Swap Plan API

use serde::Serialize;

/// This is a model struct for the SwapPlanRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SwapPlanRequest {
    /// The ID of the new subscription plan.
    pub new_plan_id: String,
}
