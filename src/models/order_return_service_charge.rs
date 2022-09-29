//! Model struct for OrderReturnServiceCharge type

use serde::{Deserialize, Serialize};

use super::{enums::OrderServiceChargeCalculationPhase, Money, OrderLineItemAppliedTax};

/// Represents the service charge applied to the original order.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderReturnServiceCharge {
    /// A unique ID that identifies the return service charge only within this order.
    pub uid: Option<String>,
    /// The service charge `uid` from the order containing the original service charge.
    /// `source_service_charge_uid` is `None` for unlinked returns.
    pub source_service_charge_uid: Option<String>,
    /// The name of the service charge.
    pub name: Option<String>,
    /// The catalog object ID of the associated [OrderServiceCharge].
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this service charge references.
    pub catalog_version: Option<i64>,
    /// The percentage of the service charge, as a string representation of a decimal number. For
    /// example, a value of "7.25" corresponds to a percentage of 7.25%.
    ///
    /// Either `percentage` or `amount_money` should be set, but not both.
    pub percentage: Option<String>,
    /// The amount of a non-percentage-based service charge.
    ///
    /// Either `percentage` or `amount_money` should be set, but not both.
    pub amount_money: Option<Money>,
    /// **Read only** The amount of money applied to the order by the service charge, including any
    /// inclusive tax amounts, as calculated by Square.
    ///
    /// * For fixed-amount service charges, `applied_money` is equal to `amount_money`.
    /// * For percentage-based service charges, `applied_money` is the money calculated using the
    /// percentage.
    pub applied_money: Option<Money>,
    /// **Read only** The total amount of money to collect for the service charge.
    ///
    /// NOTE: If an inclusive tax is applied to the service charge, `total_money` does not equal
    /// `applied_money` plus `total_tax_money` because the inclusive tax amount is already included
    /// in both `applied_money` and `total_tax_money`.
    pub total_money: Option<Money>,
    /// **Read only** The total amount of tax money to collect for the service charge.
    pub total_tax_money: Option<Money>,
    /// **Read only** The calculation phase after which to apply the service charge.
    pub calculation_phase: Option<OrderServiceChargeCalculationPhase>,
    /// Indicates whether the surcharge can be taxed. Service charges calculated in the
    /// `TOTAL_PHASE` cannot be marked as taxable.
    pub taxable: Option<bool>,
    /// The list of references to `OrderReturnTax` entities applied to the
    /// `OrderReturnServiceCharge`. Each `OrderLineItemAppliedTax` has a `tax_uid` that references
    /// the `uid` of a top-level `OrderReturnTax` that is being applied to the
    /// `OrderReturnServiceCharge`. On reads, the applied amount is populated.
    pub applied_taxes: Option<Vec<OrderLineItemAppliedTax>>,
}
