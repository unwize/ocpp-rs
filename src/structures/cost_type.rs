use serde::{Deserialize, Serialize};
use crate::enums::cost_kind_enum_type::CostKindEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// CostType is used by: Common::ConsumptionCostType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CostType {
    /// Required. The kind of cost referred to in the message element amount.
    pub cost_kind: CostKindEnumType,
    /// Required. The estimated or actual cost per kWh.
    pub amount: i32, // integer
    /// Optional. Values: -3..3. The amountMultiplier defines the exponent to base 10 (dec).
    /// The final value is determined by: amount * 10 ^ amountMultiplier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_multiplier: Option<i32>, // integer
}

impl Default for CostType {
    fn default() -> CostType {
        Self {
            cost_kind: CostKindEnumType::CarbonDioxideEmission,
            amount: 0,
            amount_multiplier: None,
        }
    }
}

impl OcppEntity for CostType {
    /// Validates the fields of CostType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        // Validate amount_multiplier
        if let Some(amount_multiplier) = self.amount_multiplier {
            e.check_bounds("amount_multiplier", -3, 3, amount_multiplier);
        }

        e.build("CostType")
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let cost = CostType {
            cost_kind: CostKindEnumType::RelativePricePercentage,
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
            cost_kind: CostKindEnumType::RenewableGenerationPercentage,
            amount: 100,
            amount_multiplier: None,
        };
        assert!(cost_minimal.validate().is_ok());

        let cost_with_multiplier = CostType {
            cost_kind: CostKindEnumType::CarbonDioxideEmission,
            amount: 250,
            amount_multiplier: Some(3), // Valid
        };
        assert!(cost_with_multiplier.validate().is_ok());

        let cost_with_min_multiplier = CostType {
            cost_kind: CostKindEnumType::CarbonDioxideEmission,
            amount: 250,
            amount_multiplier: Some(-3), // Valid
        };
        assert!(cost_with_min_multiplier.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_amount_multiplier_low() {
        let cost = CostType {
            cost_kind: CostKindEnumType::RelativePricePercentage,
            amount: 100,
            amount_multiplier: Some(-4), // Invalid
        };
        let err = cost.validate().unwrap_err();
        if let OcppError::StructureValidationError { related: source, .. } = err {
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
            cost_kind: CostKindEnumType::RelativePricePercentage,
            amount: 100,
            amount_multiplier: Some(4), // Invalid
        };
        let err = cost.validate().unwrap_err();
        if let OcppError::StructureValidationError { related: source, .. } = err {
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
