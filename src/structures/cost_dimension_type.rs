use crate::enums::cost_dimension_enum_type::CostDimensionEnumType;
use crate::errors::OcppError;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Volume consumed of cost dimension.
/// Used by: Common::ChargingPeriodType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CostDimensionType {
    /// Required. Type of cost dimension: energy, power, time, etc.
    pub r#type: CostDimensionEnumType,
    /// Required. Volume of the dimension consumed, measured according to the dimension type.
    pub volume: f64, // decimal
}

impl OcppEntity for CostDimensionType {
    fn validate(&self) -> Result<(), OcppError> {
        Ok(())
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let cost_dimension = CostDimensionType {
            r#type: CostDimensionEnumType::Energy,
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
            r#type: CostDimensionEnumType::MaxPower,
            volume: 50.0,
        };
        assert!(cost_dimension.validate().is_ok());

        let cost_dimension_zero_volume = CostDimensionType {
            r#type: CostDimensionEnumType::ChargingTime,
            volume: 0.0,
        };
        assert!(cost_dimension_zero_volume.validate().is_ok());
    }
}
