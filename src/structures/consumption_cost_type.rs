use serde::{Deserialize, Serialize};
use crate::errors::OcppError;

/// ConsumptionCostType is used by: Common::SalesTariffEntryType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumptionCostType {
    /// Required. The lowest level of consumption that defines the starting point of this consumption block.
    /// The block interval extends to the start of the next interval.
    pub start_value: f64, // decimal
    /// Required. This field contains the cost details.
    /// Cardinality 1..3
    pub cost: Vec<CostType>, // TODO: Implement CostType
}

impl ConsumptionCostType {
    /// Validates the fields of ConsumptionCostType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // No explicit constraints for start_value other than its type (decimal).
        // If there were, they would be added here (e.g., self.start_value >= 0.0).

        // Validate cost cardinality
        if self.cost.is_empty() || self.cost.len() > 3 {
            errors.push(OcppError::FieldValidationError {
                field: "cost".to_string(),
                source: vec![OcppError::FieldCardinalityError {
                    cardinality: self.cost.len() as i32,
                    lower: 1,
                    upper: 3,
                }],
            });
        }
        // TODO: If CostType had its own validate method, iterate and call it here.
        // for (i, c) in self.cost.iter().enumerate() {
        //     if let Err(e) = c.validate() {
        //         errors.push(OcppError::FieldValidationError {
        //             field: format!("cost[{}]", i),
        //             source: vec![e],
        //         });
        //     }
        // }


        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "ConsumptionCostType".to_string(),
                source: errors,
            })
        }
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let consumption_cost = ConsumptionCostType {
            start_value: 0.0,
            cost: vec!["cost_placeholder_1".to_string()], // Placeholder
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
            cost: vec!["cost1".to_string()],
        };
        assert!(consumption_cost_single.validate().is_ok());

        let consumption_cost_max_cardinality = ConsumptionCostType {
            start_value: 100.0,
            cost: vec!["cost1".to_string(), "cost2".to_string(), "cost3".to_string()],
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
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "cost");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_cost_too_many() {
        let consumption_cost = ConsumptionCostType {
            start_value: 0.0,
            cost: vec!["c".to_string(); 4], // Invalid cardinality
        };
        let err = consumption_cost.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "cost");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }
}
