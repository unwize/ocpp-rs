use serde::{Deserialize, Serialize};
use crate::errors::OcppError;

/// CostType is used by: Common::ConsumptionCostType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CostType {
    /// Required. The kind of cost referred to in the message element amount.
    pub cost_kind: CostKindEnumType, // TODO: Implement CostKindEnumType
    /// Required. The estimated or actual cost per kWh.
    pub amount: i32, // integer
    /// Optional. Values: -3..3. The amountMultiplier defines the exponent to base 10 (dec).
    /// The final value is determined by: amount * 10 ^ amountMultiplier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_multiplier: Option<i32>, // integer
}

impl CostType {
    /// Validates the fields of CostType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Validate amount_multiplier
        if let Some(multiplier) = self.amount_multiplier {
            if multiplier < -3 || multiplier > 3 {
                errors.push(OcppError::FieldValidationError {
                    field: "amount_multiplier".to_string(),
                    source: vec![OcppError::FieldBoundsError {
                        value: multiplier.to_string(),
                        lower: "-3".to_string(),
                        upper: "3".to_string(),
                    }],
                });
            }
        }

        // No validation for 'cost_kind' without its enum definition.
        // No explicit constraints for 'amount' other than its type (integer).

        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "CostType".to_string(),
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
        let cost = CostType {
            cost_kind: "RelativePrice".to_string(), // Placeholder
            amount: 1500,
            amount_multiplier: Some(-2),
        };

        let serialized = serde_json::to_string(&cost).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: CostType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cost, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let cost_minimal = CostType {
            cost_kind: "AbsolutePrice".to_string(),
            amount: 100,
            amount_multiplier: None,
        };
        assert!(cost_minimal.validate().is_ok());

        let cost_with_multiplier = CostType {
            cost_kind: "FlatRate".to_string(),
            amount: 250,
            amount_multiplier: Some(3), // Valid
        };
        assert!(cost_with_multiplier.validate().is_ok());

        let cost_with_min_multiplier = CostType {
            cost_kind: "FlatRate".to_string(),
            amount: 250,
            amount_multiplier: Some(-3), // Valid
        };
        assert!(cost_with_min_multiplier.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_amount_multiplier_low() {
        let cost = CostType {
            cost_kind: "RelativePrice".to_string(),
            amount: 100,
            amount_multiplier: Some(-4), // Invalid
        };
        let err = cost.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "amount_multiplier");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_invalid_amount_multiplier_high() {
        let cost = CostType {
            cost_kind: "RelativePrice".to_string(),
            amount: 100,
            amount_multiplier: Some(4), // Invalid
        };
        let err = cost.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "amount_multiplier");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }
}
