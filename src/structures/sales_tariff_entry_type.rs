use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::consumption_cost_type::ConsumptionCostType;
use crate::structures::relative_time_interval_type::RelativeTimeIntervalType;
use crate::traits::OcppEntity;

/// Defines a single sales tariff entry.
/// Used by: Common:SalesTariffType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffEntryType {
    /// Optional. Defines the price level of this SalesTariffEntry (referring to NumPriceLevels). Small values for the ePriceLevel represent a cheaper TariffEntry. Large values for the ePriceLevel represent a more expensive TariffEntry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_price_level: Option<i32>,
    /// Required. Defines the time interval the SalesTariffEntry is valid for, based upon relative times.
    pub relative_time_interval: RelativeTimeIntervalType,
    /// Optional. Defines additional means for further relative price information and/or alternative costs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_cost: Option<Vec<ConsumptionCostType>>,
}

impl Default for SalesTariffEntryType {
    fn default() -> Self {
        Self {
            e_price_level: None,
            relative_time_interval: Default::default(),
            consumption_cost: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for SalesTariffEntryType {
    /// Validates the fields of SalesTariffEntryType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(level) = self.e_price_level {
            e.check_bounds("e_price_level", 0, i32::MAX, level);
        }
        e.check_member("relative_time_interval", &self.relative_time_interval);
        if let Some(costs) = &self.consumption_cost {
            e.check_cardinality("consumption_cost", 0, 3, &costs.iter());
            for (i, cost) in costs.iter().enumerate() {
                e.check_member(&format!("consumption_cost[{}]", i), cost);
            }
        }

        e.build("SalesTariffEntryType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let entry = SalesTariffEntryType {
            e_price_level: None,
            relative_time_interval: Default::default(),
            consumption_cost: Some(vec![Default::default(); 3]),
        };
        assert!(entry.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let entry = SalesTariffEntryType {
            e_price_level: None,
            relative_time_interval: RelativeTimeIntervalType {
                start: 0,
                duration: None,
            },
            consumption_cost: None,
        };
        assert!(entry.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_e_price_level_negative() {
        let mut entry = SalesTariffEntryType::default();
        entry.e_price_level = Some(-1);
        let result = entry.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "e_price_level");
            }
        }
    }

    #[test]
    fn test_validate_failure_consumption_cost_too_many() {
        let mut entry = SalesTariffEntryType::default();
        entry.consumption_cost = Some(vec![Default::default(); 4]);
        let result = entry.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            println!("{:#?}", related);
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "consumption_cost");
            }
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = SalesTariffEntryType::default();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: SalesTariffEntryType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
