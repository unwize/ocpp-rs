use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// An entry in price schedule over time for which EV is willing to discharge.
/// Used by: Common::EVAbsolutePriceScheduleEntryType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EVPriceRuleType {
    /// Required. Cost per kwh.
    pub energy_fee: f64,
    /// Required. The EnergyFee applies between this value and the value of the PowerRangeStart of the subsequent EVPriceRule. If the power is below this value, the EnergyFee of the previous EVPriceRule applies. Negative values are used for discharging.
    pub power_range_start: f64,
}

impl Default for EVPriceRuleType {
    fn default() -> EVPriceRuleType {
        Self {
            energy_fee: 0.0,
            power_range_start: 0.0,
        }
    }
}

impl OcppEntity for EVPriceRuleType {
    /// Validates the fields of EVPriceRuleType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let e = StructureValidationBuilder::new();
        // All fields are required with no additional constraints specified, so no validation is needed.
        e.build("EVPriceRuleType")
    }
}
