use serde::{Deserialize, Serialize};
use crate::errors::OcppError;

/// Volume consumed of cost dimension.
/// Used by: Common::ChargingPeriodType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CostDimensionType {
    /// Required. Type of cost dimension: energy, power, time, etc.
    pub r#type: CostDimensionEnumType, // TODO: Implement CostDimensionEnumType (using r#type because 'type' is a keyword)
    /// Required. Volume of the dimension consumed, measured according to the dimension type.
    pub volume: f64, // decimal
}

impl CostDimensionType {
    /// Validates the fields of CostDimensionType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // No explicit constraints for 'type' without its enum definition.
        // No explicit constraints for 'volume' other than its type (decimal).
        // If there were, they would be added here (e.g., self.volume >= 0.0).

        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "CostDimensionType".to_string(),
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
        let cost_dimension = CostDimensionType {
            r#type: "Energy".to_string(), // Placeholder
            volume: 123.45,
        };

        let serialized = serde_json::to_string(&cost_dimension).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: CostDimensionType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cost_dimension, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let cost_dimension = CostDimensionType {
            r#type: "Power".to_string(),
            volume: 50.0,
        };
        assert!(cost_dimension.validate().is_ok());

        let cost_dimension_zero_volume = CostDimensionType {
            r#type: "Time".to_string(),
            volume: 0.0,
        };
        assert!(cost_dimension_zero_volume.validate().is_ok());
    }
}
