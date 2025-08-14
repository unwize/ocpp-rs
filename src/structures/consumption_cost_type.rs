use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::cost_type::CostType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// ConsumptionCostType is used by: Common::SalesTariffEntryType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConsumptionCostType {
    /// Required. The lowest level of consumption that defines the starting point of this consumption block.
    /// The block interval extends to the start of the next interval.
    pub start_value: f64, // decimal
    /// Required. This field contains the cost details.
    /// Cardinality 1..3
    pub cost: Vec<CostType>,
}

impl Default for ConsumptionCostType {
    fn default() -> ConsumptionCostType {
        Self {
            start_value: 0.0,
            cost: vec![CostType::default()],
        }
    }
}

impl OcppEntity for ConsumptionCostType {
    /// Validates the fields of ConsumptionCostType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        e.check_cardinality("cost", 1, 3, &self.cost.iter());
        e.check_iter_member("cost", self.cost.iter());
        e.build("ConsumptionCostType")
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let consumption_cost = ConsumptionCostType {
            start_value: 0.0,
            cost: vec![CostType::default()], // Placeholder
        };

        let serialized = serde_json::to_string(&consumption_cost).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ConsumptionCostType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(consumption_cost, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let consumption_cost_single = ConsumptionCostType {
            start_value: 10.5,
            cost: vec![CostType::default()],
        };
        assert!(consumption_cost_single.validate().is_ok());

        let consumption_cost_max_cardinality = ConsumptionCostType {
            start_value: 100.0,
            cost: vec![CostType::default(); 3],
        };
        assert!(consumption_cost_max_cardinality.validate().is_ok());
    }

    #[test]
    fn test_validation_cost_empty() {
        let consumption_cost = ConsumptionCostType {
            start_value: 0.0,
            cost: vec![], // Invalid cardinality
        };
        let err = consumption_cost.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["cost"]);
    }

    #[test]
    fn test_validation_cost_too_many() {
        let consumption_cost = ConsumptionCostType {
            start_value: 0.0,
            cost: vec![CostType::default(); 4], // Invalid cardinality
        };
        let err = consumption_cost.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["cost"]);
    }
}
