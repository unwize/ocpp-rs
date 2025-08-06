use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::errors::OcppError::FieldRelationshipError;
use crate::traits::OcppEntity;

/// Used by: Common::DERCurveType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HysteresisType {
    /// Optional. High value for return to normal operation after a grid event, in absolute value. This value adopts the same unit as defined by yUnit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hysteresis_high: Option<f64>,
    /// Optional. Low value for return to normal operation after a grid event, in absolute value. This value adopts the same unit as defined by yUnit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hysteresis_low: Option<f64>,
    /// Optional. Delay in seconds, once grid parameter within HysteresisLow and HysteresisHigh, for the EV to return to normal operation after a grid event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hysteresis_delay: Option<f64>,
    /// Optional. Set default rate of change (ramp rate %/s) for the EV to return to normal operation after a grid event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hysteresis_gradient: Option<f64>,
}

impl OcppEntity for HysteresisType {
    /// Validates the fields of HysteresisType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(high) = self.hysteresis_high {
            if let Some(low) = self.hysteresis_low {
                if low > high {
                    e.push_relation_error("high", "low", "hysteresis_high must be greater than hysteresis_low!");
                }
            }
        }

        e.build("HysteresisType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let hysteresis_type_full = HysteresisType {
            hysteresis_high: Some(51.0),
            hysteresis_low: Some(49.0),
            hysteresis_delay: Some(2.5),
            hysteresis_gradient: Some(0.1),
        };
        assert!(hysteresis_type_full.validate().is_ok());

        let hysteresis_type_minimal = HysteresisType {
            hysteresis_high: None,
            hysteresis_low: None,
            hysteresis_delay: None,
            hysteresis_gradient: None,
        };
        assert!(hysteresis_type_minimal.validate().is_ok());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct_full = HysteresisType {
            hysteresis_high: Some(51.0),
            hysteresis_low: Some(49.0),
            hysteresis_delay: Some(2.5),
            hysteresis_gradient: Some(0.1),
        };

        let serialized_full = serde_json::to_string(&original_struct_full).unwrap();
        let deserialized_full: HysteresisType = serde_json::from_str(&serialized_full).unwrap();
        assert_eq!(original_struct_full, deserialized_full);

        let original_struct_minimal = HysteresisType {
            hysteresis_high: None,
            hysteresis_low: None,
            hysteresis_delay: None,
            hysteresis_gradient: None,
        };
        let serialized_minimal = serde_json::to_string(&original_struct_minimal).unwrap();
        let deserialized_minimal: HysteresisType = serde_json::from_str(&serialized_minimal).unwrap();
        assert_eq!(original_struct_minimal, deserialized_minimal);
    }
}